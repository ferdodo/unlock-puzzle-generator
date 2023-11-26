use super::random_number::random_number;
use super::block::Block;
use super::is_bit_move_legal::is_bit_move_legal;
use super::is_latch_move_legal::is_latch_move_legal;
use super::puzzle::Puzzle;
use super::bit::Bit;
use super::latch:Latch;
use super::there_is_a_big_vertical_block_in_the_upper_right::there_is_a_big_vertical_block_in_the_upper_right;
use super::there_is_few_empty_slots::there_is_few_empty_slots;
use super::generate_bits::generate_bits;

pub fn generate_puzzle() -> Puzzle {
    let mut puzzle: Puzzle = Puzzle {
        block: Block { x: 0, y: 0, w: 6, h: 6 },
        latch: Latch { block: Block { x: 0, y: 2, w: 2, h: 1 } },
        bits: vec![],
    };

    puzzle.bits = generate_bits(puzzle);
    let mut bit_moved: std::collections::HashSet<usize> = std::collections::HashSet::new();

    for moves in 0..5000 {
        if puzzle_is_good(puzzle, &bit_moved) {
            return puzzle;
        }

        if moves > 5000 {
            return generate_puzzle();
        }

        let move_bit = random_number(0, 100) < 90;

        if move_bit {
            let bit_index = random_number(0, puzzle.bits.len());
            let bit = &puzzle.bits[bit_index];
            let move_direction = random_number(0, 100) > 50;
            let move_sign = if random_number(0, 100) > 50 { 1 } else { -1 };

            let move_bit_event = MoveBitEvent {
                id: bit.id,
                x: bit.block.x + if move_direction { move_sign } else { 0 },
                y: bit.block.y + if move_direction { 0 } else { move_sign },
            };

            let moved_block = Block {
                x: bit.block.x + if move_direction { move_sign } else { 0 },
                y: bit.block.y + if move_direction { 0 } else { move_sign },
                w: bit.block.w,
                h: bit.block.h,
            };

            if is_bit_move_legal(&puzzle, bit.id, &moved_block) {
                bit_moved.insert(bit.id);
                puzzle.bits[bit_index].block = moved_block;
            }
        } else {
            let direction = random_number(0, 100) < 50;
            let move_latch_event = MoveLatchEvent {
                x: puzzle.latch.block.x + if direction { 1 } else { -1 },
            };

            let moved_block = Block {
                x: puzzle.latch.block.x + if direction { 1 } else { -1 },
                y: puzzle.latch.block.y,
                w: puzzle.latch.block.w,
                h: puzzle.latch.block.h,
            };

            if is_latch_move_legal(&puzzle, &moved_block) {
                puzzle.latch.block.x = move_latch_event.x;
            }
        }
    }

    puzzle
}

fn puzzle_is_good(puzzle: Puzzle, bit_moved: &std::collections::HashSet<usize>) -> bool {
    puzzle.latch.block.x == 0
        && there_is_a_big_vertical_block_in_the_upper_right(puzzle)
        && there_is_few_empty_slots(puzzle)
        && bit_moved.len() == puzzle.bits.len()
}
