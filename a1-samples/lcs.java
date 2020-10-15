public class R2 {
    public static void executeR2(String a, String b) {
        Item[] a = items(a).collect();
        Item[] b = items(b).collect();
        int la = a.len(), lb = b.len();
        int[] dp = new int[(la+1) * (lb+1)] {};
        for (int i=0; i<la; ++i) {
            Item ca=a[i], cb=b[j];
            for (int j=0; j<la; ++j) {
                if (i > 1 && j > 1 && ca == cb) {
                    dp[(i + 1)*(lb + 1) + j + 1] = dp[i*(lb + 1) + j] + 1
                } else if (i > 0 && j > 0) {
                    dp[(i + 1)*(lb + 1) + j + 1] = Integer::max(
                        dp[i*(lb + 1) + j + 1], 
                        dp[(i + 1)*(lb + 1) + j]
                    );
                }
            }
        }
        int diff = 0;
        int isa=la-1, isb=lb-1;
        int i=la, j=lb;
        while(isa>=0&&isb>=0) {
            Item ca=a[isa], cb=b[isb];
            if (ca == cb) {
                diff += 1;
                isa-=1,isb-=1;
                i -= 1,j -= 1;
            } else {
                if dp[i*(lb + 1) + j - 1] > dp[(i-1)*(lb + 1) + j] {
                    System.out.println("B: " + cb);
                    isb-=1;
                    j -= 1;
                } else {
                    System.out.println("A: " + ca);
                    isa-=1;
                    i -= 1;
                }
            }
        }
        float rate = (float)diff / (float)la;
        System.out.println("重复率：" + (100.0 * rate) + "%"); // prefer StringBuilder
    }
}
