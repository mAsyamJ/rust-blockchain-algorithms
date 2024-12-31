use sha2::{Sha256, Digest};

// sha2 is a dictionary that provides function hashing SHA-256
// Sha 256: Structure hashing SHA-256
// Digest: Trait for hasing function in update and finalization

// Making sha256_hash function
fn sha256_hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
// For input data string that will be count (hash)
    // Process: Sha254::new() to create a new instance in SHA-256 hasher
    // hasher.update(data): Process input data to hashing algorithm
    // hasher.finalize(): Creating final result of final hash in byte array
    // format!("{:x}", ...): Convert byte array to string format hex
// Output: string from hash value

//Building Merkle Tree
    // Input: list hash string from leaf node
fn build_merkle_tree(mut leaves: Vec<String>) -> String {
    // If case 1: Only has one node in list leaves then it is merkle root
    if leaves.len() == 1 {
        return leaves[0].clone();
    }
    //  elif case 2: If node in leaves is odd (ganjil)
    if leaves.len()  %2==1 {
        leaves.push(leaves.last().unwrap().clone());
    }
        //leaves last(): Take out the last element
        //leaves.push(...): Duplicate the last element so total node is odd (genap)
        
    // 2. Combine two Node (2-2) 
    let mut parent_nodes = Vec::new();
    for i in (0..leaves.len()).step_by(2) {
        let combined = format!("{}{}", leaves[i], leaves[i+1]);
        parent_nodes.push(sha256_hash(&combined));
        // .step_by(2): Loop in spaced 2 to pair the nodes
        // format!("{}{}", leaves[i], leaves[i+1]): combine hash from
            // two nodes into one string
        // sha256_hash: Count hash from combined nodes
        // parent_nodes.push: Saving output hash to list parent_nodes (Big one)
        
    }
    
    // Rekursif to build the tree
    build_merkle_tree(parent_nodes) // Tak back erkle Root value into string
    
}


// Main Function
// 1. Leaf Node List
fn main() {
    let leaves = vec![
        "12","34","56","78",
        "90","23","45","67"
        // list of leaf node as string (leaves)
        // change it here.
    ];
    
    // Hash of every leaf node
    let hashed_leaves: Vec<String> = leaves
        .iter() //Making iterator on every element in leaves
        .map(|&leaf|sha256_hash(leaf)) // Apply sha256_hash function to every element
        .collect(); //Convert map output into list (vec<String>)
        // Output: Hash List from leaaf node
        
    // Print Hash from every Leaf Node
    println!("Hash setiap daun: ");
    for (i, hash) in hashed_leaves.iter().enumerate() {
        println!("L{}: {}", i+1, hash);
        //enumerate(): Giving index on every elemtn from hashed_leaves
    }
    
    // Making Merkle Tree and Print it
    let merkle_root = build_merkle_tree(hashed_leaves);
    
    println!("\nMerkle Root:");
    println!("{}", merkle_root);
}