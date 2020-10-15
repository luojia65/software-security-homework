fn execute_r2(a: String, b: String) {
    let a = items(a).collect::<Vec<_>>();
    let b = items(b).collect::<Vec<_>>();

    // println!("{:?}", a);

    // LCS算法第一步，得到子序列索引数组
    let (la, lb) = (a.len(), b.len());
    let mut dp = vec![0; (a.len() + 1) * (b.len() + 1)];
    for (i, ca) in a.iter().enumerate() {
        for (j, cb) in b.iter().enumerate() {
            if i > 1 && j > 1 && ca == cb {
                dp[(i + 1)*(lb + 1) + j + 1] = dp[i*(lb + 1) + j] + 1
            } else if i > 0 && j > 0 {
                dp[(i + 1)*(lb + 1) + j + 1] = usize::max(
                    dp[i*(lb + 1) + j + 1], 
                    dp[(i + 1)*(lb + 1) + j]
                );
            }
        }
    }
    // for i in 0..=la {
    //     println!("{:?}", &dp[i*(lb+1)..(i+1)*(lb+1)]);
    // }

    let mut diff = 0;

    // LCS第二步，得到有差别的元素。注意的是这个算法是从后往前倒着输出的
    let mut sa = a.iter().rev().peekable();
    let mut sb = b.iter().rev().peekable();
    let mut i = la;
    let mut j = lb;
    while let (Some(ca), Some(cb)) = (sa.peek(), sb.peek()) {
        if i == 0 && j == 0 {
            break
        }
        if ca == cb {
            diff += 1;
            sa.next();
            sb.next();
            i -= 1;
            j -= 1;
        } else {
            if dp[i*(lb + 1) + j - 1] > dp[(i-1)*(lb + 1) + j] {
                println!("B: {:?}", cb);
                sb.next();
                j -= 1;
            } else {
                println!("A: {:?}", ca);
                sa.next();
                i -= 1;
            }
        }
    }

    let rate = diff as f32 / a.len() as f32;
    println!("重复率：{}%", 100.0 * rate);
}
