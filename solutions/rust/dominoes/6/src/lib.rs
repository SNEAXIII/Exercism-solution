type Domino = (u8, u8);
type Placed = (usize, bool);
type Stack = Vec<Placed>;
#[must_use]
fn recurse_fill(
    stack: &mut Stack,
    used: &mut [bool],
    next_domino: &[Stack],
    dominos: &[Domino],
) -> bool {
    let (current_index, is_reversed) = match stack.last() {
        None => return false,
        Some(value) => *value,
    };
    let current_domino = dominos[current_index];
    let current_value = if is_reversed {
        current_domino.0
    } else {
        current_domino.1
    } as usize;
    for domino in &next_domino[current_value] {
        if used[domino.0] {
            continue;
        }
        used[domino.0] = true;
        stack.push(*domino);
        if stack.len() == dominos.len() {
            return true;
        }
        if recurse_fill(stack, used, next_domino, dominos) {
            return true;
        }
        used[domino.0] = false;
        stack.pop();
    }
    false
}

pub fn chain(dominos: &[Domino]) -> Option<Vec<Domino>> {
    if dominos.is_empty() {
        return Some(vec![]);
    } else if dominos.len() == 1 && dominos[0].0 == dominos[0].1 {
        return Some(vec![dominos[0]]);
    }

    let mut next_domino: [Stack; 7] = Default::default();
    let mut stack: Stack = Vec::with_capacity(dominos.len());
    stack.push((0, false));

    for (index, &(a, b)) in dominos[1..].iter().enumerate() {
        next_domino[a as usize].push((index + 1, false));
        next_domino[b as usize].push((index + 1, true));
    }

    let mut used = vec![false; dominos.len()];
    used[0] = true;

    if !recurse_fill(&mut stack, &mut used, &next_domino, dominos) {
        return None;
    }

    let mut result: Vec<Domino> = Vec::new();
    for (index, is_reverse) in stack {
        let mut new_domino: Domino = dominos[index];
        if is_reverse {
            new_domino = (new_domino.1, new_domino.0)
        }
        result.push(new_domino);
    }

    if result[0].0 != result[dominos.len() - 1].1 {
        return None;
    }

    Some(result)
}