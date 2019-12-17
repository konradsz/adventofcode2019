use std::iter;

fn get_pattern(length: usize, offset: usize) -> Vec<i32> {
	let pattern = [0, 1, 0, -1];

	let mut output = Vec::new();
	for digit in pattern.iter().cycle() {
		let dig_iter = iter::repeat(digit);
		dig_iter.take(offset).for_each(|d| output.push(*d));

		if output.len() > length {
			break;
		}
	}

	output.remove(0);
	output.truncate(length);

	output
}

fn main() {
    let input = "59708372326282850478374632294363143285591907230244898069506559289353324363446827480040836943068215774680673708005813752468017892971245448103168634442773462686566173338029941559688604621181240586891859988614902179556407022792661948523370366667688937217081165148397649462617248164167011250975576380324668693910824497627133242485090976104918375531998433324622853428842410855024093891994449937031688743195134239353469076295752542683739823044981442437538627404276327027998857400463920633633578266795454389967583600019852126383407785643022367809199144154166725123539386550399024919155708875622641704428963905767166129198009532884347151391845112189952083025";
    let input_length = input.len();

    let mut input: Vec<i32> = input.chars().map(|c| (c.to_digit(10).unwrap()) as i32).collect();

    for _ in 0..100 { 
	    let mut new_input = Vec::new();
	    for offset in 0..input_length {
	    	let pattern = get_pattern(input_length, offset + 1);
	    	let sum = input.iter().zip(pattern.iter()).map(|(d1, d2)| d1 * d2).sum::<i32>();
	    	new_input.push((sum % 10).abs());
	    }
	    input = new_input;
    }

    input.iter().take(8).for_each(|e| print!("{}", e));
}
