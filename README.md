# DiffEqR

ODE System solver for Python, implemented in Rust. This package calls the [diffeq](https://lib.rs/crates/diffeq) crate written in Rust. For the moment, only the Explicit Runge-Kutta of order 5(4) algorithm is used.

## Usage

The syntax is the same as [scipy's solve_ivp](https://docs.scipy.org/doc/scipy/reference/generated/scipy.integrate.solve_ivp.html):

```python
import diffeqr

dydt = lambda t, y: [y[i-1] for i in range(len(y))] # dy0/dt = y1, dy1/dt = y2, dy2/dt=y0

result = diffeqr.solve_ivp(fun=dydt, t_span=(0.0, 1.0), y0=[1.0, 2.0, 1.0j])
```

Arguments:

* `fun`: A function (or lambda) with two arguments, corresponding to the right-hand side of `dy/dt = f(t, y)`. `t` is a real number, and `y` a vector (list/tuple) of real and/or complex numbers.
* `t_span`: Tuple containing the initial and final values of `t`, `t=t0` and `t=tf`.
* `y0`: Initial value of `y` at `t=t0`.
* All other arguments are, at the moment, ignored.

This method returns the solution for `y` at `t=tf`. Unlike scipy's version, it does not return `y` at intermediate points or any other information, although this might change in future versions.
