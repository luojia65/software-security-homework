int XFunc(int i,int j)
{
    int m = 3;
    int n = 4;
    char szBuf[8] = {0}; 
    strncpy(szBuf, "This is a overflow buffer!", 100);
    m = i;
    n = j;
    return 8;
}
