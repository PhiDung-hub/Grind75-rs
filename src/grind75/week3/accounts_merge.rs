// Problem: https://leetcode.com/problems/accounts-merge/
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::structs::UnionFind;

pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut email_idx_map: HashMap<&String, usize> = HashMap::new();
    let mut uf = UnionFind::new(accounts.len());

    for (i, account) in accounts.iter().enumerate() {
        for email in account.iter().skip(1) {
            if let Some(&j) = email_idx_map.get(email) {
                uf.union(i, j);
            } else {
                email_idx_map.insert(email, i);
            }
        }
    }

    let mut account_emails_map: HashMap<usize, Vec<String>> = HashMap::new();
    for (email, i) in email_idx_map.into_iter() {
        let key = uf.find(i);
        match account_emails_map.entry(key) {
            Entry::Vacant(e) => {
                e.insert(vec![accounts[i][0].clone(), email.clone()]);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().push(email.clone());
            }
        }
    }

    let merged_accounts = Vec::from_iter(account_emails_map.values().cloned().collect::<Vec<_>>());
    merged_accounts
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    fn sort_answer(_vec_vec: &mut Vec<Vec<String>>) {
        // each vector of _vec_vec should always contains more than 1 element
        _vec_vec.sort_by_key(|v| match v.first() {
            Some(value) => value.clone(),
            None => panic!("invalid input"),
        });
        for vec in _vec_vec.iter_mut() {
            vec.sort();
        }
    }

    #[test]
    fn example_1() {
        let accounts = vec![
            ["John", "johnsmith@mail.com", "john_newyork@mail.com"].map(String::from).to_vec(),
            ["John", "johnsmith@mail.com", "john00@mail.com"].map(String::from).to_vec(),
            ["Mary", "mary@mail.com"].map(String::from).to_vec(),
            ["John", "johnnybravo@mail.com"].map(String::from).to_vec(),
        ];
        let mut expected_result = vec![
            ["John", "john00@mail.com", "john_newyork@mail.com", "johnsmith@mail.com"]
                .map(String::from)
                .to_vec(),
            ["Mary", "mary@mail.com"].map(String::from).to_vec(),
            ["John", "johnnybravo@mail.com"].map(String::from).to_vec(),
        ];
        let mut result = accounts_merge(accounts);

        sort_answer(&mut result);
        sort_answer(&mut expected_result);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn example_2() {
        let accounts = vec![
            ["John", "johnsmith@mail.com", "john_newyork@mail.com"].map(String::from).to_vec(),
            ["John", "johnsmith@mail.com", "john00@mail.com"].map(String::from).to_vec(),
            ["Mary", "mary@mail.com"].map(String::from).to_vec(),
            ["John", "johnsmith@mail.com", "johnnybravo@mail.com"].map(String::from).to_vec(),
        ];
        let mut expected_result = vec![
            ["John", "john00@mail.com", "john_newyork@mail.com", "johnsmith@mail.com", "johnnybravo@mail.com"]
                .map(String::from)
                .to_vec(),
            ["Mary", "mary@mail.com"].map(String::from).to_vec(),
        ];
        let mut result = accounts_merge(accounts);

        sort_answer(&mut result);
        sort_answer(&mut expected_result);

        assert_eq!(result, expected_result);
    }
}
