use z3::Solver;
use z3::ast::Int;

pub fn foo() {
    let solver = Solver::new();

    // define the variable.
    let x = Int::new_const("x");

    // define the equation
    solver.assert((&x + 4).eq(7));

    // run the solver
    _ = solver.check();
    let model = solver.get_model().unwrap();

    println!("{model:?}");
}
