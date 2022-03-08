use crate::fixture::Fixture;
use crate::generation::prototyping::{
    PrototypesGlweCiphertext, PrototypesGlweSecretKey, PrototypesPlaintextVector,
};
use crate::generation::synthesizing::{
    SynthesizesGlweCiphertext, SynthesizesGlweSecretKey, SynthesizesPlaintextVector,
};
use crate::generation::{IntegerPrecision, Maker};
use crate::raw::generation::RawUnsignedIntegers;
use crate::raw::statistical_test::assert_noise_distribution;
use concrete_commons::dispersion::Variance;
use concrete_commons::parameters::{GlweDimension, PolynomialSize};
use concrete_core::prelude::{
    GlweCiphertextDecryptionEngine, GlweCiphertextEntity, GlweSecretKeyEntity,
    PlaintextVectorEntity,
};

/// A fixture for the types implementing the `GlweCiphertextTensorProductEngine` trait.
pub struct GlweCiphertextTensorProductFixture;

#[derive(Debug)]
pub struct GlweCiphertextTensorProductParameters {
    pub noise: Variance,
    pub glwe_dimension: GlweDimension,
    pub polynomial_size: PolynomialSize,
}

impl<Precision, Engine, PlaintextVector, SecretKey, Ciphertext>
Fixture<Precision, Engine, (PlaintextVector, SecretKey, Ciphertext)>
for GlweCiphertextTensorProductFixture
    where
        Precision: IntegerPrecision,
        Engine: GlweCiphertextTensorProductEngine<SecretKey, PlaintextVector, Ciphertext>,
        PlaintextVector: PlaintextVectorEntity,
        SecretKey: GlweSecretKeyEntity,
        Ciphertext: GlweCiphertextEntity<KeyDistribution = SecretKey::KeyDistribution>,
        Maker: SynthesizesPlaintextVector<Precision, PlaintextVector>
        + SynthesizesGlweSecretKey<Precision, SecretKey>
        + SynthesizesGlweCiphertext<Precision, Ciphertext>,
{
    type Parameters = GlweCiphertextTensorProductParameters;
    type RepetitionPrototypes = (
        <Maker as PrototypesGlweSecretKey<Precision, SecretKey::KeyDistribution>>::GlweSecretKeyProto,
    );
    type SamplePrototypes =
    (<Maker as PrototypesPlaintextVector<Precision>>::PlaintextVectorProto,
     <Maker as PrototypesGlweCiphertext<Precision, SecretKey::KeyDistribution>>::GlweCiphertextProto,
     <Maker as PrototypesPlaintextVector<Precision>>::PlaintextVectorProto,
     <Maker as PrototypesGlweCiphertext<Precision, SecretKey::KeyDistribution>>::GlweCiphertextProto);

    type PreExecutionContext = (SecretKey, PlaintextVector, PlaintextVector, Ciphertext, Ciphertext);
    type PostExecutionContext = (SecretKey, Ciphertext);
    type Criteria = (Variance,);
    type Outcome = (Vec<Precision::Raw>, Vec<Precision::Raw>);

    fn generate_parameters_iterator() -> Box<dyn Iterator<Item = Self::Parameters>> {
        Box::new(
            vec![
                GlweCiphertextTensorProductParameters {
                    noise: Variance(0.00000001),
                    glwe_dimension: GlweDimension(200),
                    polynomial_size: PolynomialSize(256),
                },
                GlweCiphertextTensorProductParameters {
                    noise: Variance(0.00000001),
                    glwe_dimension: GlweDimension(1),
                    polynomial_size: PolynomialSize(256),
                },
                GlweCiphertextTensorProductParameters {
                    noise: Variance(0.00000001),
                    glwe_dimension: GlweDimension(200),
                    polynomial_size: PolynomialSize(1),
                },
            ]
                .into_iter(),
        )
    }

    fn generate_random_repetition_prototypes(
        parameters: &Self::Parameters,
        maker: &mut Maker,
    ) -> Self::RepetitionPrototypes {
        let proto_secret_key =
            maker.new_glwe_secret_key(parameters.glwe_dimension, parameters.polynomial_size);
        (proto_secret_key,)
    }

    fn generate_random_sample_prototypes(
        parameters: &Self::Parameters,
        maker: &mut Maker,
        _repetition_proto: &Self::RepetitionPrototypes,
    ) -> Self::SamplePrototypes {
        let raw_plaintext_vector1 = Precision::Raw::uniform_vec(parameters.polynomial_size.0);
        let proto_plaintext_vector1 =
            maker.transform_raw_vec_to_plaintext_vector(raw_plaintext_vector1.as_slice());
        let proto_ciphertext1 = maker.encrypt_plaintext_vector_to_glwe_ciphertext(
            proto_secret_key,
            &proto_plaintext_vector,
            parameters.noise,
        );
        let raw_plaintext_vector2 = Precision::Raw::uniform_vec(parameters.polynomial_size.0);
        let proto_plaintext_vector2 =
            maker.transform_raw_vec_to_plaintext_vector(raw_plaintext_vector2.as_slice());
        let proto_ciphertext2 = maker.encrypt_plaintext_vector_to_glwe_ciphertext(
            proto_secret_key,
            &proto_plaintext_vector,
            parameters.noise,
        );
        (proto_plaintext_vector1, proto_ciphertext1, proto_plaintext_vector2, proto_ciphertext2)
    }

    fn prepare_context(
        _parameters: &Self::Parameters,
        maker: &mut Maker,
        repetition_proto: &Self::RepetitionPrototypes,
        sample_proto: &Self::SamplePrototypes,
    ) -> Self::PreExecutionContext {
        let (proto_secret_key,) = repetition_proto;
        let (proto_plaintext_vector1,
            proto_ciphertext1,
            proto_plaintext_vector2,
            proto_ciphertext2) = sample_proto;
        (
            maker.synthesize_glwe_secret_key(proto_secret_key),
            maker.synthesize_plaintext_vector(proto_plaintext_vector1),
            maker.synthesize_ciphertext_vectir(proto_ciphertext1),
            maker.synthesize_plaintext_vector(proto_plaintext_vector2),
            maker.synthesize_ciphertext_vector(proto_ciphertext2)
        )
    }

    // TODO: need to make sure the engine is used correctly for Tensor Product
    fn execute_engine(
        parameters: &Self::Parameters,
        engine: &mut Engine,
        context: Self::PreExecutionContext,
    ) -> Self::PostExecutionContext {
        let (secret_key, plaintext_vector) = context;
        let ciphertext = unsafe {
            engine.tensor_product(proto_ciphertext1, proto_ciphertext2)
        };
        (secret_key, ciphertext)
    }

    fn process_context(
        _parameters: &Self::Parameters,
        maker: &mut Maker,
        repetition_proto: &Self::RepetitionPrototypes,
        sample_proto: &Self::SamplePrototypes,
        context: Self::PostExecutionContext,
    ) -> Self::Outcome {
        let (proto_plaintext_vector,) = sample_proto;
        let (proto_secret_key,) = repetition_proto;
        let (secret_key, plaintext_vector, ciphertext) = context;
        let proto_output_ciphertext = maker.unsynthesize_glwe_ciphertext(&ciphertext);
        let proto_output_plaintext_vector = maker.decrypt_glwe_ciphertext_to_plaintext_vector(
            proto_secret_key,
            &proto_output_ciphertext,
        );
        maker.destroy_plaintext_vector(plaintext_vector);
        maker.destroy_glwe_secret_key(secret_key);
        maker.destroy_glwe_ciphertext(ciphertext);
        (
            maker.transform_plaintext_vector_to_raw_vec(proto_plaintext_vector),
            maker.transform_plaintext_vector_to_raw_vec(&proto_output_plaintext_vector),
        )
    }

    fn compute_criteria(
        parameters: &Self::Parameters,
        _maker: &mut Maker,
        _repetition_proto: &Self::RepetitionPrototypes,
    ) -> Self::Criteria {
        (parameters.noise,)
    }

    fn verify(criteria: &Self::Criteria, outputs: &[Self::Outcome]) -> bool {
        let (means, actual): (Vec<_>, Vec<_>) = outputs.iter().cloned().unzip();
        let means: Vec<Precision::Raw> = means.into_iter().flatten().collect();
        let actual: Vec<Precision::Raw> = actual.into_iter().flatten().collect();
        assert_noise_distribution(&actual, means.as_slice(), criteria.0)
    }
}