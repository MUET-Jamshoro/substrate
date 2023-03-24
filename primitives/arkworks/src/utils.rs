use ark_ec::pairing::{MillerLoopOutput, Pairing};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};
use ark_std::{io::Cursor, vec, vec::Vec};

pub fn serialize_result(result: impl CanonicalSerialize) -> Vec<u8> {
	let mut serialized_result = vec![0u8; result.serialized_size(Compress::No)];
	let mut cursor = Cursor::new(&mut serialized_result[..]);
	result.serialize_uncompressed(&mut cursor).unwrap();
	serialized_result
}

pub fn deserialize_argument<Field: CanonicalDeserialize>(argument: &Vec<u8>) -> Field {
	let cursor = Cursor::new(argument);
	Field::deserialize_with_mode(cursor, Compress::No, Validate::No).unwrap()
}

pub fn multi_miller_loop_generic<Curve: Pairing>(
	a_vec: Vec<Vec<u8>>,
	b_vec: Vec<Vec<u8>>,
) -> Result<Vec<u8>, ()> {
	let g1: Vec<_> = a_vec
		.iter()
		.map(|elem| deserialize_argument::<<Curve as Pairing>::G1Affine>(elem))
		.collect();
	let g2: Vec<_> = b_vec
		.iter()
		.map(|elem| deserialize_argument::<<Curve as Pairing>::G2Affine>(elem))
		.collect();

	let result = Curve::multi_miller_loop(g1, g2);

	Ok(serialize_result(result.0))
}

pub fn final_exponentiation_generic<Curve: Pairing>(target: Vec<u8>) -> Result<Vec<u8>, ()> {
	let target = deserialize_argument::<<Curve as Pairing>::TargetField>(&target);

	let result = Curve::final_exponentiation(MillerLoopOutput(target));

	match result {
		Some(result) => Ok(serialize_result(result)),
		None => Err(()),
	}
}

pub fn msm_g1_generic<Curve: Pairing>(bases: Vec<Vec<u8>>, scalars: Vec<Vec<u8>>) -> Vec<u8> {
	let bases: Vec<_> = bases
		.iter()
		.map(|a| deserialize_argument::<<Curve as Pairing>::G1Affine>(a))
		.collect();
	let scalars: Vec<_> =
		scalars.iter().map(|a| deserialize_argument::<Curve::ScalarField>(a)).collect();

	let result =
		<<Curve as Pairing>::G1 as ark_ec::VariableBaseMSM>::msm(&bases, &scalars).unwrap();

	serialize_result(result)
}

pub fn msm_g2_generic<Curve: Pairing>(bases: Vec<Vec<u8>>, scalars: Vec<Vec<u8>>) -> Vec<u8> {
	let bases: Vec<_> = bases
		.iter()
		.map(|a| deserialize_argument::<<Curve as Pairing>::G2Affine>(a))
		.collect();
	let scalars: Vec<_> =
		scalars.iter().map(|a| deserialize_argument::<Curve::ScalarField>(a)).collect();

	let result =
		<<Curve as Pairing>::G2 as ark_ec::VariableBaseMSM>::msm(&bases, &scalars).unwrap();

	serialize_result(result)
}
