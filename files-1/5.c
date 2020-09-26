int AFunc()
{   
    int a=1,b=2,c=3;
    printf("%d"); // 字符串里给了，但是参数里没有
    printf("%d%d",a,b,c); // 参数多了，字符串没给
    printf("%d%d%d",a,b,c); // 这个是对的
    return 8;
}
