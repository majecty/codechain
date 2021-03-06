// Copyright 2018-2019 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

extern crate codechain_crypto as ccrypto;
extern crate codechain_key as ckey;
extern crate codechain_types as ctypes;
extern crate codechain_vm as cvm;
extern crate primitives;

mod common;

use ccrypto::{BLAKE_EMPTY, BLAKE_NULL_RLP};
use ckey::NetworkId;
use common::TestClient;
use ctypes::transaction::{AssetOutPoint, AssetTransferInput, ShardTransaction};
use cvm::{execute, RuntimeError, ScriptResult, VMConfig};
use cvm::{Instruction, TimelockType};
use primitives::{H160, H256};

#[test]
fn simple_success() {
    let client = TestClient::default();
    let transaction = ShardTransaction::TransferAsset {
        network_id: NetworkId::default(),
        burns: Vec::new(),
        inputs: Vec::new(),
        outputs: Vec::new(),
    };
    let input = AssetTransferInput {
        prev_out: AssetOutPoint {
            tracker: Default::default(),
            index: 0,
            asset_type: H160::default(),
            shard_id: 0,
            quantity: 0,
        },
        timelock: None,
        lock_script: Vec::new(),
        unlock_script: Vec::new(),
    };
    assert_eq!(
        execute(&[], &[], &[Instruction::Push(1)], &transaction, VMConfig::default(), &input, false, &client, 0, 0),
        Ok(ScriptResult::Unlocked)
    );

    assert_eq!(
        execute(&[], &[], &[Instruction::Success], &transaction, VMConfig::default(), &input, false, &client, 0, 0),
        Ok(ScriptResult::Unlocked)
    );
}

#[test]
fn simple_failure() {
    let client = TestClient::default();
    let transaction = ShardTransaction::TransferAsset {
        network_id: NetworkId::default(),
        burns: Vec::new(),
        inputs: Vec::new(),
        outputs: Vec::new(),
    };
    let input = AssetTransferInput {
        prev_out: AssetOutPoint {
            tracker: Default::default(),
            index: 0,
            asset_type: H160::default(),
            shard_id: 0,
            quantity: 0,
        },
        timelock: None,
        lock_script: Vec::new(),
        unlock_script: Vec::new(),
    };
    assert_eq!(
        execute(&[Instruction::Push(0)], &[], &[], &transaction, VMConfig::default(), &input, false, &client, 0, 0),
        Ok(ScriptResult::Fail)
    );
    assert_eq!(
        execute(&[], &[], &[Instruction::Fail], &transaction, VMConfig::default(), &input, false, &client, 0, 0),
        Ok(ScriptResult::Fail)
    );
}

#[test]
fn simple_burn() {
    let client = TestClient::default();
    let transaction = ShardTransaction::TransferAsset {
        network_id: NetworkId::default(),
        burns: Vec::new(),
        inputs: Vec::new(),
        outputs: Vec::new(),
    };
    let input = AssetTransferInput {
        prev_out: AssetOutPoint {
            tracker: Default::default(),
            index: 0,
            asset_type: H160::default(),
            shard_id: 0,
            quantity: 0,
        },
        timelock: None,
        lock_script: Vec::new(),
        unlock_script: Vec::new(),
    };
    assert_eq!(
        execute(&[], &[], &[Instruction::Burn], &transaction, VMConfig::default(), &input, false, &client, 0, 0),
        Ok(ScriptResult::Burnt)
    );
}

#[test]
fn underflow() {
    let client = TestClient::default();
    let transaction = ShardTransaction::TransferAsset {
        network_id: NetworkId::default(),
        burns: Vec::new(),
        inputs: Vec::new(),
        outputs: Vec::new(),
    };
    let input = AssetTransferInput {
        prev_out: AssetOutPoint {
            tracker: Default::default(),
            index: 0,
            asset_type: H160::default(),
            shard_id: 0,
            quantity: 0,
        },
        timelock: None,
        lock_script: Vec::new(),
        unlock_script: Vec::new(),
    };
    assert_eq!(
        execute(&[], &[], &[Instruction::Pop], &transaction, VMConfig::default(), &input, false, &client, 0, 0),
        Err(RuntimeError::StackUnderflow)
    );
}

#[test]
fn out_of_memory() {
    let client = TestClient::default();
    let transaction = ShardTransaction::TransferAsset {
        network_id: NetworkId::default(),
        burns: Vec::new(),
        inputs: Vec::new(),
        outputs: Vec::new(),
    };
    let input = AssetTransferInput {
        prev_out: AssetOutPoint {
            tracker: Default::default(),
            index: 0,
            asset_type: H160::default(),
            shard_id: 0,
            quantity: 0,
        },
        timelock: None,
        lock_script: Vec::new(),
        unlock_script: Vec::new(),
    };
    let config = VMConfig {
        max_memory: 2,
    };
    assert_eq!(
        execute(
            &[Instruction::Push(0), Instruction::Push(1), Instruction::Push(2)],
            &[],
            &[],
            &transaction,
            config,
            &input,
            false,
            &client,
            0,
            0
        ),
        Err(RuntimeError::OutOfMemory)
    );
}

#[test]
fn invalid_unlock_script() {
    let client = TestClient::default();
    let transaction = ShardTransaction::TransferAsset {
        network_id: NetworkId::default(),
        burns: Vec::new(),
        inputs: Vec::new(),
        outputs: Vec::new(),
    };
    let input = AssetTransferInput {
        prev_out: AssetOutPoint {
            tracker: Default::default(),
            index: 0,
            asset_type: H160::default(),
            shard_id: 0,
            quantity: 0,
        },
        timelock: None,
        lock_script: Vec::new(),
        unlock_script: Vec::new(),
    };
    assert_eq!(
        execute(&[Instruction::Nop], &[], &[], &transaction, VMConfig::default(), &input, false, &client, 0, 0),
        Ok(ScriptResult::Fail)
    );
}

#[test]
fn conditional_burn() {
    let client = TestClient::default();
    let transaction = ShardTransaction::TransferAsset {
        network_id: NetworkId::default(),
        burns: Vec::new(),
        inputs: Vec::new(),
        outputs: Vec::new(),
    };
    let input = AssetTransferInput {
        prev_out: AssetOutPoint {
            tracker: Default::default(),
            index: 0,
            asset_type: H160::default(),
            shard_id: 0,
            quantity: 0,
        },
        timelock: None,
        lock_script: Vec::new(),
        unlock_script: Vec::new(),
    };
    let lock_script = vec![Instruction::Eq, Instruction::Dup, Instruction::Jnz(1), Instruction::Burn];
    assert_eq!(
        execute(
            &[Instruction::Push(0)],
            &[vec![0]],
            &lock_script,
            &transaction,
            VMConfig::default(),
            &input,
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Unlocked)
    );
    assert_eq!(
        execute(
            &[Instruction::Push(0)],
            &[vec![1]],
            &lock_script,
            &transaction,
            VMConfig::default(),
            &input,
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Burnt)
    );
}

#[test]
fn _blake256() {
    let client = TestClient::default();
    let transaction = ShardTransaction::TransferAsset {
        network_id: NetworkId::default(),
        burns: Vec::new(),
        inputs: Vec::new(),
        outputs: Vec::new(),
    };
    let input = AssetTransferInput {
        prev_out: AssetOutPoint {
            tracker: Default::default(),
            index: 0,
            asset_type: H160::default(),
            shard_id: 0,
            quantity: 0,
        },
        timelock: None,
        lock_script: Vec::new(),
        unlock_script: Vec::new(),
    };
    let lock_script = vec![Instruction::Blake256, Instruction::Eq];
    assert_eq!(
        execute(
            &[],
            &[vec![], BLAKE_EMPTY.to_vec()],
            &lock_script,
            &transaction,
            VMConfig::default(),
            &input,
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Unlocked)
    );
    assert_eq!(
        execute(
            &[],
            &[vec![], BLAKE_NULL_RLP.to_vec()],
            &lock_script,
            &transaction,
            VMConfig::default(),
            &input,
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Fail)
    );
    assert_eq!(
        execute(
            &[],
            &[vec![0x80], BLAKE_NULL_RLP.to_vec()],
            &lock_script,
            &transaction,
            VMConfig::default(),
            &input,
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Unlocked)
    );
    assert_eq!(
        execute(
            &[],
            &[vec![0x80], BLAKE_EMPTY.to_vec()],
            &lock_script,
            &transaction,
            VMConfig::default(),
            &input,
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Fail)
    );
}

#[test]
fn _ripemd160() {
    let client = TestClient::default();
    let transaction = ShardTransaction::TransferAsset {
        network_id: NetworkId::default(),
        burns: Vec::new(),
        inputs: Vec::new(),
        outputs: Vec::new(),
    };
    let input = AssetTransferInput {
        prev_out: AssetOutPoint {
            tracker: Default::default(),
            index: 0,
            asset_type: H160::default(),
            shard_id: 0,
            quantity: 0,
        },
        timelock: None,
        lock_script: Vec::new(),
        unlock_script: Vec::new(),
    };
    const RIPEMD160_EMPTY: H160 = H160([
        0x9c, 0x11, 0x85, 0xa5, 0xc5, 0xe9, 0xfc, 0x54, 0x61, 0x28, 0x08, 0x97, 0x7e, 0xe8, 0xf5, 0x48, 0xb2, 0x25,
        0x8d, 0x31,
    ]);
    const RIPEMD160_NULL_RLP: H160 = H160([
        0xb4, 0x36, 0x44, 0x1e, 0x6b, 0xb8, 0x82, 0xfe, 0x0a, 0x0f, 0xa0, 0x32, 0x0c, 0xb2, 0xd9, 0x7d, 0x96, 0xb4,
        0xd1, 0xbc,
    ]);
    let lock_script = vec![Instruction::Ripemd160, Instruction::Eq];
    assert_eq!(
        execute(
            &[],
            &[vec![], RIPEMD160_EMPTY.to_vec()],
            &lock_script,
            &transaction,
            VMConfig::default(),
            &input,
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Unlocked)
    );
    assert_eq!(
        execute(
            &[],
            &[vec![], RIPEMD160_NULL_RLP.to_vec()],
            &lock_script,
            &transaction,
            VMConfig::default(),
            &input,
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Fail)
    );
    assert_eq!(
        execute(
            &[],
            &[vec![0x80], RIPEMD160_NULL_RLP.to_vec()],
            &lock_script,
            &transaction,
            VMConfig::default(),
            &input,
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Unlocked)
    );
    assert_eq!(
        execute(
            &[],
            &[vec![0x80], RIPEMD160_EMPTY.to_vec()],
            &lock_script,
            &transaction,
            VMConfig::default(),
            &input,
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Fail)
    );
}

#[test]
fn _sha256() {
    let client = TestClient::default();
    let transaction = ShardTransaction::TransferAsset {
        network_id: NetworkId::default(),
        burns: Vec::new(),
        inputs: Vec::new(),
        outputs: Vec::new(),
    };
    let input = AssetTransferInput {
        prev_out: AssetOutPoint {
            tracker: Default::default(),
            index: 0,
            asset_type: H160::default(),
            shard_id: 0,
            quantity: 0,
        },
        timelock: None,
        lock_script: Vec::new(),
        unlock_script: Vec::new(),
    };
    const SHA256_EMPTY: H256 = H256([
        0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9, 0x24, 0x27, 0xae,
        0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52, 0xb8, 0x55,
    ]);
    const SHA256_NULL_RLP: H256 = H256([
        0x76, 0xbe, 0x8b, 0x52, 0x8d, 0x00, 0x75, 0xf7, 0xaa, 0xe9, 0x8d, 0x6f, 0xa5, 0x7a, 0x6d, 0x3c, 0x83, 0xae,
        0x48, 0x0a, 0x84, 0x69, 0xe6, 0x68, 0xd7, 0xb0, 0xaf, 0x96, 0x89, 0x95, 0xac, 0x71,
    ]);
    let lock_script = vec![Instruction::Sha256, Instruction::Eq];
    assert_eq!(
        execute(
            &[],
            &[vec![], SHA256_EMPTY.to_vec()],
            &lock_script,
            &transaction,
            VMConfig::default(),
            &input,
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Unlocked)
    );
    assert_eq!(
        execute(
            &[],
            &[vec![], SHA256_NULL_RLP.to_vec()],
            &lock_script,
            &transaction,
            VMConfig::default(),
            &input,
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Fail)
    );
    assert_eq!(
        execute(
            &[],
            &[vec![0x80], SHA256_NULL_RLP.to_vec()],
            &lock_script,
            &transaction,
            VMConfig::default(),
            &input,
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Unlocked)
    );
    assert_eq!(
        execute(
            &[],
            &[vec![0x80], SHA256_EMPTY.to_vec()],
            &lock_script,
            &transaction,
            VMConfig::default(),
            &input,
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Fail)
    );
}

#[test]
fn _keccak256() {
    let client = TestClient::default();
    let transaction = ShardTransaction::TransferAsset {
        network_id: NetworkId::default(),
        burns: Vec::new(),
        inputs: Vec::new(),
        outputs: Vec::new(),
    };
    let input = AssetTransferInput {
        prev_out: AssetOutPoint {
            tracker: Default::default(),
            index: 0,
            asset_type: H160::default(),
            shard_id: 0,
            quantity: 0,
        },
        timelock: None,
        lock_script: Vec::new(),
        unlock_script: Vec::new(),
    };
    const KECCAK256_EMPTY: H256 = H256([
        0xc5, 0xd2, 0x46, 0x01, 0x86, 0xf7, 0x23, 0x3c, 0x92, 0x7e, 0x7d, 0xb2, 0xdc, 0xc7, 0x03, 0xc0, 0xe5, 0x00,
        0xb6, 0x53, 0xca, 0x82, 0x27, 0x3b, 0x7b, 0xfa, 0xd8, 0x04, 0x5d, 0x85, 0xa4, 0x70,
    ]);
    const KECCAK256_NULL_RLP: H256 = H256([
        0x56, 0xe8, 0x1f, 0x17, 0x1b, 0xcc, 0x55, 0xa6, 0xff, 0x83, 0x45, 0xe6, 0x92, 0xc0, 0xf8, 0x6e, 0x5b, 0x48,
        0xe0, 0x1b, 0x99, 0x6c, 0xad, 0xc0, 0x01, 0x62, 0x2f, 0xb5, 0xe3, 0x63, 0xb4, 0x21,
    ]);
    let lock_script = vec![Instruction::Keccak256, Instruction::Eq];
    assert_eq!(
        execute(
            &[],
            &[vec![], KECCAK256_EMPTY.to_vec()],
            &lock_script,
            &transaction,
            VMConfig::default(),
            &input,
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Unlocked)
    );
    assert_eq!(
        execute(
            &[],
            &[vec![], KECCAK256_NULL_RLP.to_vec()],
            &lock_script,
            &transaction,
            VMConfig::default(),
            &input,
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Fail)
    );
    assert_eq!(
        execute(
            &[],
            &[vec![0x80], KECCAK256_NULL_RLP.to_vec()],
            &lock_script,
            &transaction,
            VMConfig::default(),
            &input,
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Unlocked)
    );
    assert_eq!(
        execute(
            &[],
            &[vec![0x80], KECCAK256_EMPTY.to_vec()],
            &lock_script,
            &transaction,
            VMConfig::default(),
            &input,
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Fail)
    );
}

#[cfg(test)]
fn dummy_tx() -> ShardTransaction {
    ShardTransaction::TransferAsset {
        network_id: NetworkId::default(),
        burns: Vec::new(),
        inputs: Vec::new(),
        outputs: Vec::new(),
    }
}

fn dummy_input() -> AssetTransferInput {
    AssetTransferInput {
        prev_out: AssetOutPoint {
            tracker: Default::default(),
            index: 0,
            asset_type: H160::default(),
            shard_id: 0,
            quantity: 0,
        },
        timelock: None,
        lock_script: Vec::new(),
        unlock_script: Vec::new(),
    }
}

#[test]
fn timelock_invalid_value() {
    assert_eq!(
        execute(
            &[],
            &[],
            &[Instruction::PushB(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]), Instruction::ChkTimelock(TimelockType::Block)],
            &dummy_tx(),
            VMConfig::default(),
            &dummy_input(),
            false,
            &TestClient::default(),
            0,
            0
        ),
        Err(RuntimeError::TypeMismatch)
    )
}

#[test]
fn timelock_block_number_success() {
    let client = TestClient::new(None, None);
    assert_eq!(
        execute(
            &[],
            &[],
            &[Instruction::PushB(vec![10]), Instruction::ChkTimelock(TimelockType::Block)],
            &dummy_tx(),
            VMConfig::default(),
            &dummy_input(),
            false,
            &client,
            10,
            0
        ),
        Ok(ScriptResult::Unlocked)
    )
}

#[test]
fn timelock_block_number_fail() {
    let client = TestClient::new(None, None);
    assert_eq!(
        execute(
            &[],
            &[],
            &[Instruction::PushB(vec![10]), Instruction::ChkTimelock(TimelockType::Block)],
            &dummy_tx(),
            VMConfig::default(),
            &dummy_input(),
            false,
            &client,
            9,
            0
        ),
        Ok(ScriptResult::Fail)
    )
}

#[test]
fn timelock_block_timestamp_success() {
    // 0x5BD02BF2, 2018-10-24T08:23:14+00:00
    let client = TestClient::new(None, None);
    assert_eq!(
        execute(
            &[],
            &[],
            &[Instruction::PushB(vec![0x00, 0x5B, 0xD0, 0x2B, 0xF2]), Instruction::ChkTimelock(TimelockType::Time)],
            &dummy_tx(),
            VMConfig::default(),
            &dummy_input(),
            false,
            &client,
            0,
            1_540_369_394
        ),
        Ok(ScriptResult::Unlocked)
    )
}

#[test]
fn timelock_block_timestamp_fail() {
    // 0x5BD02BF1, 2018-10-24T08:23:13+00:00
    let client = TestClient::new(None, None);
    assert_eq!(
        execute(
            &[],
            &[],
            &[Instruction::PushB(vec![0x00, 0x5B, 0xD0, 0x2B, 0xF2]), Instruction::ChkTimelock(TimelockType::Time)],
            &dummy_tx(),
            VMConfig::default(),
            &dummy_input(),
            false,
            &client,
            0,
            1_540_369_393
        ),
        Ok(ScriptResult::Fail)
    )
}

#[test]
fn timelock_block_age_fail_due_to_none() {
    let client = TestClient::new(None, None);
    assert_eq!(
        execute(
            &[],
            &[],
            &[Instruction::PushB(vec![1]), Instruction::ChkTimelock(TimelockType::BlockAge)],
            &dummy_tx(),
            VMConfig::default(),
            &dummy_input(),
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Fail)
    )
}

#[test]
fn timelock_block_age_fail() {
    let client = TestClient::new(Some(4), None);
    assert_eq!(
        execute(
            &[],
            &[],
            &[Instruction::PushB(vec![5]), Instruction::ChkTimelock(TimelockType::BlockAge)],
            &dummy_tx(),
            VMConfig::default(),
            &dummy_input(),
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Fail)
    )
}

#[test]
fn timelock_time_age_fail_due_to_none() {
    let client = TestClient::new(None, None);
    assert_eq!(
        execute(
            &[],
            &[],
            &[Instruction::PushB(vec![0x27, 0x8D, 0x00]), Instruction::ChkTimelock(TimelockType::TimeAge)],
            &dummy_tx(),
            VMConfig::default(),
            &dummy_input(),
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Fail)
    )
}

#[test]
fn timelock_block_age_success() {
    let client = TestClient::new(Some(5), None);
    assert_eq!(
        execute(
            &[],
            &[],
            &[Instruction::PushB(vec![5]), Instruction::ChkTimelock(TimelockType::BlockAge)],
            &dummy_tx(),
            VMConfig::default(),
            &dummy_input(),
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Unlocked)
    )
}

#[test]
fn timelock_time_age_fail() {
    // 0x278D00 seconds = 2592000 seconds = 30 days
    let client = TestClient::new(None, Some(2_591_999));
    assert_eq!(
        execute(
            &[],
            &[],
            &[Instruction::PushB(vec![0x27, 0x8D, 0x00]), Instruction::ChkTimelock(TimelockType::TimeAge)],
            &dummy_tx(),
            VMConfig::default(),
            &dummy_input(),
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Fail)
    )
}

#[test]
fn timelock_time_age_success() {
    let client = TestClient::new(None, Some(2_592_000));
    assert_eq!(
        execute(
            &[],
            &[],
            &[Instruction::PushB(vec![0x27, 0x8D, 0x00]), Instruction::ChkTimelock(TimelockType::TimeAge)],
            &dummy_tx(),
            VMConfig::default(),
            &dummy_input(),
            false,
            &client,
            0,
            0
        ),
        Ok(ScriptResult::Unlocked)
    )
}

#[test]
fn copy_stack_underflow() {
    let client = TestClient::default();
    let transaction = ShardTransaction::TransferAsset {
        network_id: NetworkId::default(),
        burns: Vec::new(),
        inputs: Vec::new(),
        outputs: Vec::new(),
    };
    let input = AssetTransferInput {
        prev_out: AssetOutPoint {
            tracker: Default::default(),
            index: 0,
            asset_type: H160::default(),
            shard_id: 0,
            quantity: 0,
        },
        timelock: None,
        lock_script: Vec::new(),
        unlock_script: Vec::new(),
    };
    assert_eq!(
        execute(&[], &[], &[Instruction::Copy(1)], &transaction, VMConfig::default(), &input, false, &client, 0, 0),
        Err(RuntimeError::StackUnderflow)
    );
}
