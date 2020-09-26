int AFunc(int i,int j)
{
    int m = 3;
    int n = 4;
    char szBuf[16] = {0}; 
    strcpy(szBuf, "This is a overflow buffer!");
    strcpy(szBuf, "This is not!");
    m = i;
    n = j;
    return 8;
}
