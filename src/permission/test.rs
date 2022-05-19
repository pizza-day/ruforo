use super::*;
use rand::Rng;

#[test]
fn test_mask_can()
{
    let mut mask: PermissionMask = Default::default();
    mask.groups[0] = 0b0101u64;
    
    assert_eq!(mask.can(0, 0), true);
    assert_eq!(mask.can(0, 1), false);
    assert_eq!(mask.can(0, 2), true);
    assert_eq!(mask.can(0, 3), false);
    assert_eq!(mask.can(0, 4), false);
    
    assert_eq!(mask.can(1, 0), false);
    assert_eq!(mask.can(2, 1), false);
    assert_eq!(mask.can(3, 2), false);
    assert_eq!(mask.can(4, 3), false);
}

#[test]
fn test_permission_add()
{
    let mut cat = PermissionCategory::default();

    for i in 0..PERM_LIMIT {
        match cat.add_permission(rand::thread_rng().gen_range(1..999) as i32) {
            Ok(p) => assert_eq!(p.position as u32, i),
            Err(_) => assert!(false, "Unexpected overflowing permission category"),
        }
        
    }

    let pr = cat.add_permission((PERM_LIMIT + 1) as i32);
    assert!(pr.is_err(), "Did not contain an error.");
}

#[test]
fn test_set_join()
{
    let group1a = PermissionGroupValues {
        yes:   0b0011u64,
        no:    0b0000u64,
        never: 0b0000u64,
    };
    let group1b = PermissionGroupValues {
        yes:   0b0000u64,
        no:    0b1000u64,
        never: 0b0100u64,
    };
    let group2a = PermissionGroupValues {
        yes:   0b0000u64,
        no:    0b0010u64,
        never: 0b0001u64,
    };
    let group2b = PermissionGroupValues {
        yes:   0b1100u64,
        no:    0b0000u64,
        never: 0b0000u64,
    };

    let mut set1: PermissionSet = Default::default();
    set1.groups[0] = group1a;
    set1.groups[1] = group1b;

    let mut set2: PermissionSet = Default::default();
    set2.groups[0] = group2a;
    set2.groups[1] = group2b;

    let set3 = set1.join(&set2);

    assert_eq!(set1.groups[0].yes, 0b0011u64);
    assert_eq!(set2.groups[1].yes, 0b1100u64);
    assert_eq!(set3.groups[0].yes, 0b0010u64);
    assert_eq!(set3.groups[1].yes, 0b1000u64);
}

#[test]
fn test_values_join_combines_never()
{
    let group1 = PermissionGroupValues {
        yes:   0b100u64,
        no:    0b000u64,
        never: 0b010u64,
    };
    let group2 = PermissionGroupValues {
        yes:   0b011u64,
        no:    0b000u64,
        never: 0b100u64,
    };
    let group3 = group1.join(&group2);

    assert_eq!(group3.yes,   0b0001u64);
    assert_eq!(group3.no,    0b0000u64);
    assert_eq!(group3.never, 0b0110u64);
}

#[test]
fn test_values_join_overwrites_no()
{
    let group1 = PermissionGroupValues {
        yes:   0b111u64,
        no:    0b000u64,
        never: 0b000u64,
    };
    let group2 = PermissionGroupValues {
        yes:   0b0000u64,
        no:    0b0010u64,
        never: 0b1000u64,
    };
    let group3 = group1.join(&group2);

    assert_eq!(group3.yes,   0b0111u64);
    assert_eq!(group3.no,    0b0000u64);
    assert_eq!(group3.never, 0b1000u64);
}

#[test]
fn test_values_stack_negatives()
{
    let group1 = PermissionGroupValues {
        yes:   0b01100u64,
        no:    0b00010u64,
        never: 0b00001u64,
    };
    let group2 = PermissionGroupValues {
        yes:   0b10011u64,
        no:    0b01111u64,
        never: 0b01000u64,
    };
    let group3 = group1.stack(&group2);

    assert_eq!(group3.yes,   0b10100u64);
    assert_eq!(group3.no,    0b00010u64);
    assert_eq!(group3.never, 0b01001u64);
}