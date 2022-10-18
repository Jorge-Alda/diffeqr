use diffeq::ode::problem::OdeProblem;
use diffeq::error::OdeError;
use diffeq::ode::solution::OdeSolution;
use diffeq::ode::Ode;
use pyo3::prelude::*;
use pyo3::exceptions::PyTypeError;

pub struct DiffEq {
    pub init: Vec<f64>,
    pub start: f64,
    pub end: f64,
    pub num: usize,
}

impl DiffEq {
    pub fn new (init: Vec<f64>, t_span: (f64, f64)) -> Self {
        let (start, end) = t_span;
        Self {init: init, start: start, end: end, num: 2}
    }
}


pub fn to_rust(f: Py<PyAny>) -> Box<dyn Fn(f64, &Vec<f64>) -> Vec<f64>> {
    Box::new(move  |x: f64, v: &Vec<f64>| -> Vec<f64> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v2 = v.clone();
        let args = (x, v2);
        let res = f.call1(py, args).unwrap();
        res.extract::<Vec<f64>>(py).unwrap()}
    )
}


fn solve_ode<F: Fn(f64, &Vec<f64>) -> Vec<f64>>(eq: F, config: &DiffEq) -> Result<OdeSolution<f64, Vec<f64>>, OdeError> {
    let problem = OdeProblem::builder()
    .tspan_linspace(config.start, config.end, config.num)
    .fun(eq)
    .init(config.init.clone())
    .build()
    .unwrap();

    let solution = problem
    .solve(Ode::Ode45, Default::default());

    solution
}

#[pyfunction]
fn solve_real(fun: Py<PyAny>, t_span: (f64, f64), y0: Vec<f64>) ->PyResult<Vec<f64>> {
    let config = &DiffEq::new(y0, t_span);
    let solution = solve_ode(to_rust(fun),config);
    match solution {
        Ok(sol) => {
            let y = sol.yout;
            let l = y.len();
            let mut v = vec![0.0; y[l-1].len()];
            for i in 0..y[l-1].len() {
                v[i] = y[l-1][i];
            }
            Ok(v)
        },
        Err(_) => Err(PyTypeError::new_err("Error while solving the ODEs"))
    }
}


/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn diffeqr(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve_real, m)?)?;
    Ok(())
}