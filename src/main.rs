// extern crate blas;
extern crate intel_mkl_src;
use cblas::*;
fn main() {
    println!("Hello world");
    let (m, n, k) = (2, 4, 3);
    let a = vec![
        200.0, 4.0,
        2.0, 5.0,
        3.0, 6.0,
    ];
    let b = vec![
        200.0, 5.0,  9.0,
        2.0, 6.0, 10.0,
        3.0, 7.0, 200.0,
        4.0, 8.0, 12.0,
    ];
    let mut c = vec![
        2.0, 7.0,
        6.0, 2.0,
        0.0, 7.0,
        4.0, 2.0,
    ];

    unsafe {
        // sgemm == single precision floating, dgemm == double precision
        cblas::sgemm(Layout::ColumnMajor, Transpose::None, Transpose::None,
            m, n, k, 1.0, &a, m, &b, k, 1.0, &mut c, m);
    }

    println!("{:?}", c);

    println!("Starting assert");
    assert!(
        c == vec![
            40039.0, 886.0,
            448.0, 100.0,
            1214.0, 1254.0,
            856.0, 130.0,
        ]
       
    );
    println!("Ending assert");
}
