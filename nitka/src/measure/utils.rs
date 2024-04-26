use std::ops::Sub;

use near_workspaces::types::Gas;

pub fn generate_permutations<One, Two>(one: &[One], two: &[Two]) -> Vec<(One, Two)>
where
    One: Copy,
    Two: Copy,
{
    one.iter()
        .flat_map(|&item1| two.iter().map(move |&item2| (item1, item2)))
        .collect()
}

pub fn generate_permutations_3<One, Two, Three>(one: &[One], two: &[Two], three: &[Three]) -> Vec<(One, Two, Three)>
where
    One: Copy,
    Two: Copy,
    Three: Copy,
{
    one.iter()
        .flat_map(|&item1| {
            two.iter()
                .flat_map(move |&item2| three.iter().map(move |&item3| (item1, item2, item3)))
        })
        .collect()
}

pub fn values_diff<T, V>(values: T) -> Vec<V>
where
    V: Sub<Output = V> + Copy,
    T: IntoIterator<Item = V>,
{
    let vec: Vec<_> = values.into_iter().collect();
    vec.windows(2).map(|w| w[1] - w[0]).collect()
}

pub fn pretty_gas_string(gas: Gas) -> String {
    format!(
        "{} TGas {} GGas total: {}",
        gas.as_tgas(),
        strip_tgas(gas).as_ggas(),
        gas.as_gas()
    )
}

fn strip_tgas(gas: Gas) -> Gas {
    Gas::from_ggas(gas.as_ggas() - gas.as_tgas() * 1000)
}

#[cfg(test)]
mod test {
    use near_workspaces::types::Gas;

    use crate::measure::utils::{
        generate_permutations, generate_permutations_3, pretty_gas_string, strip_tgas, values_diff,
    };

    #[test]
    fn permutations() -> anyhow::Result<()> {
        assert_eq!(
            generate_permutations(&['a', 'b'], &[10, 20]),
            vec![('a', 10,), ('a', 20,), ('b', 10,), ('b', 20,),]
        );

        assert_eq!(
            generate_permutations_3(&['a', 'b'], &[10, 20], &[true, false]),
            vec![
                ('a', 10, true,),
                ('a', 10, false,),
                ('a', 20, true,),
                ('a', 20, false,),
                ('b', 10, true,),
                ('b', 10, false,),
                ('b', 20, true,),
                ('b', 20, false,),
            ]
        );

        Ok(())
    }

    #[test]
    fn pretty_gas_string_test() {
        assert_eq!(
            pretty_gas_string(Gas::from_ggas(5555)),
            "5 TGas 555 GGas total: 5555000000000"
        );
    }

    #[test]
    fn values_diff_test() {
        assert_eq!(values_diff([1, 2, 6, 50]), [1, 4, 44]);
    }

    #[test]
    fn strip_tgas_test() {
        assert_eq!(strip_tgas(Gas::from_ggas(5555)), Gas::from_ggas(555));
    }
}
