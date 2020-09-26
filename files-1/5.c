int AFunc()
{   
    int a=1,b=2,c=3,buf[100];
    printf("%d"); // 字符串里给了，但是参数里没有
    printf("%d%d",a,b,c); // 参数多了，字符串没给
    printf("%d%d%d",a,b,c); // 这个是对的

    sprintf(buf,"%d%d",a,b,c); // 格式字符串缺参数
    sprintf(buf,"%d%d%d",c); // 变量里缺参数
    sprintf(buf,"%d%d%d",a,b,c); // 这个是对的

    sprintf(buf,"%n",a); // 可能的溢出问题
    return 8;
}
