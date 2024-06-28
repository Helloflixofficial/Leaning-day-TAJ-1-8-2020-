fn data(n:i32) -> i32{
    let mut sum = 0;
    for i in  0..=n{
        sum += i;
    }
    sum
}
fn data2(n: i32) -> i32 {
    n * (n + 1) / 2
}
fn main(){
    let result = data(10);
    let result2 = data2(10);
    println!("the sum is {:#}",result);
    println!("the sum is {:#}",result2);
}




# Array
nums = [1, 2, 3]
nums.append(4)    # push to end
nums.pop()        # pop from end
nums[0]           # lookup
nums[1]
nums[2]


# HashMap / Set
hashMap = {}
hashMap["key"] = 10     # insert
print("key" in hashMap) # lookup
print(hashMap["key"])   # lookup
hashMap.pop("key")      # remove
