void execute_r2(string a, string b) 
{
    vector<item_t> a = items(a) ->collect();
    vector<item_t> b = items(b) ->collect();
    size_t la = a->len(), lb = b->len();
    vector<size_t> dp; // size = (la + 1) * (lb + 1)
    for (size_t i=0; i<la; ++i) {
        item_t ca=a[i], cb=b[j];
        for (size_t j=0; j<la; ++j) {
            if ( i > 1 && j > 1 && ca == cb ) {
                dp[ (i + 1) * (lb + 1) + j + 1] = dp[i* (lb + 1) + j] + 1
            } else if ( i > 0 && j > 0 ) {
                dp[ (i + 1) * (lb + 1) + j + 1] = MAX(
                    dp[i * (lb + 1) + j + 1], 
                    dp[ (i + 1) * (lb + 1) + j]
                );
            }
        }
    }
    size_t diff = 0;
    size_t isa=la-1, isb=lb-1;
    size_t i=la, j=lb;
    while ( isa >=0 && isb>=0 ) {
        item_t ca=a[isa], cb=b[isb];
        if (ca == cb) {
            diff += 1;
            isa-=1,isb-=1;
            i -= 1,j -= 1;
        } else {
            if dp[i* (lb + 1) + j - 1] > dp[ (i-1) * (lb + 1) + j] {
                printf("B: %c\n", cb);
                isb-=1;
                j -= 1;
            } else {
                printf("A: %c\n", ca);
                isa-=1;
                i -= 1;
            }
        }
    }
    float rate = (float)diff / (float)la;
    cout<<"重复率：{}"<<(100.0 * rate)<<"%"<<endl;
}
