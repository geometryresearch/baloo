use std::marker::PhantomData;

use ark_ff::{PrimeField, Zero};
use ark_poly::{univariate::DensePolynomial, GeneralEvaluationDomain, Polynomial};

pub struct WellFormation<F: PrimeField> {
    _f: PhantomData<F>,
}

impl<F: PrimeField> WellFormation<F> {
    pub fn prove(
        e: &DensePolynomial<F>,
        v: &DensePolynomial<F>,
        zi: &DensePolynomial<F>,
        domain_v: GeneralEvaluationDomain<F>,
        beta: F,
    ) -> DensePolynomial<F> {
        let mut lhs = v * beta;
        lhs[0] -= F::one();
        lhs = &lhs * e;

        lhs[0] += zi.evaluate(&beta) * zi.evaluate(&F::zero()).inverse().unwrap();

        let (q, r) = lhs.divide_by_vanishing_poly(domain_v).unwrap();
        assert!(r.is_zero());

        q
    }
}