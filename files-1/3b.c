int DES_Encrypt(char *plainFile, char *keyStr,char *cipherFile){  
    ElemType plainBlock[8],cipherBlock[8],keyBlock[8];  
    ElemType bKey[64];  
    ElemType subKeys[16][48]; 
    FILE *plain,*cipher;  
    int count;  
    if((plain = fopen(plainFile,"rb")) == NULL){  
  return PLAIN_FILE_OPEN_ERROR; }     
    if((cipher = fopen(cipherFile,"wb")) == NULL){  
        return CIPHER_FILE_OPEN_ERROR;  
    }  //设置密钥
      
    memcpy(keyBlock,keyStr,8);  //将密钥转换为二进制流  
    Char8ToBit64(keyBlock,bKey);   //生成子密钥  
    DES_MakeSubKeys(bKey,subKeys);  
      
    while(!feof(plain)){  
        //每次读8个字节，并返回成功读取的字节数  
        if((count = fread(plainBlock,sizeof(char),8,plain)) == 8){  
            DES_EncryptBlock(plainBlock,subKeys,cipherBlock);  
            fwrite(cipherBlock,sizeof(char),8,cipher);    
        }  
    }  
    if(count){  
        //填充  
        memset(plainBlock + count,'\0',7 - count);  
    //最后一个字符保存包括最后一个字符在内的所填充的字符数量  
        plainBlock[7] = 8 - count;  
        DES_EncryptBlock(plainBlock,subKeys,cipherBlock);  
        fwrite(cipherBlock,sizeof(char),8,cipher);  
    }  
    fclose(plain);  
    fclose(cipher);  
 return OK;  
}  
