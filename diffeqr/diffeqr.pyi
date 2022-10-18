from typing import Callable


def solve_real(fun: Callable[[float, list[float], list[float]]], t_span: tuple[float], y0: list[float]) -> list[float]:
    """Solves a real initial value problem for a system of first order ODEs

    Args:
        fun (Callable[[float, list[float], list[float]]]): Function in the right-hand side of dy/dt = F(t, y)
        t_span (tuple): Start and end values for t
        y0 (list[float]): Initial value for y at t0

    Returns:
        list[float]: Solution to the IVP at tf
    """
