use rand::thread_rng;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use std::sync::atomic::AtomicBool;

// use snarkvm_algorithms::{MerkleParameters, CRH, SNARK};

use snarkvm::dpc::{
    posw::PoSWCircuit,
    testnet2::Testnet2,
    // BlockHeader,
    // BlockHeaderMetadata,
    // PoSWError,
    BlockTemplate,
    Network,
    PoSWProof,
    PoSWScheme,
};
use snarkvm::utilities::Uniform;

type N = Testnet2;

use rand::prelude::*;

pub fn get_proof(block_template: BlockTemplate<Testnet2>, random: u64) -> PoSWProof<N> {
    let terminator = &AtomicBool::new(false);
    let rng = &mut ChaChaRng::seed_from_u64(random);
    let mut circuit = match PoSWCircuit::<Testnet2>::new(&block_template, Uniform::rand(rng)) {
        Ok(circuit) => circuit,
        Err(e) => panic!("posw circuit {}", e),
    };

    let proof = match Testnet2::posw().prove_once_unchecked(&mut circuit, terminator, rng) {
        Ok(proof) => proof,
        Err(e) => panic!("posw proof {}", e),
    };

    if Testnet2::posw().verify(
        block_template.difficulty_target(),
        &circuit.to_public_inputs(),
        &proof,
    ) {
        // Construct a block header.
        // return Ok(BlockHeader::from(
        //     block_template.previous_ledger_root(),
        //     block_template.transactions().transactions_root(),
        //     BlockHeaderMetadata::new(block_template),
        //     circuit.nonce(),
        //     proof,
        // )?);
    } else {
        dbg!("----");
    }
    proof
}

fn get_genesis_template() -> BlockTemplate<Testnet2> {
    // let difficulty_target: u64 = 18446744073709551615; // block.difficulty_target()
    let difficulty_target: u64 = 18446744073709551615;

    println!("Difficulty_target is: {:?}", difficulty_target);
    // Construct the block template.
    let block = Testnet2::genesis_block();
    let block_template = BlockTemplate::new(
        block.previous_block_hash(),
        block.height(),
        block.timestamp(),
        difficulty_target,
        block.cumulative_weight(),
        block.previous_ledger_root(),
        block.transactions().clone(),
        block
            .to_coinbase_transaction()
            .unwrap()
            .to_records()
            .next()
            .unwrap(),
    );
    block_template
}

fn main() {
    let mut rng = thread_rng();
    let genesis_template = get_genesis_template();
    for _ in 0..200 {
        let random = rng.gen_range(0..=u64::MAX);
        get_proof(genesis_template.clone(), random);
    }
}