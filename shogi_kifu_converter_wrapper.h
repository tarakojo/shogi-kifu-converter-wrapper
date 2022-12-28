#include<stdint.h>


/*
sugyan様作 shogi-kifu-converter
https://github.com/sugyan/shogi-kifu-converter

のffi用 ラッパー


dst ... 変換結果文字列の格納先(utf-8)
dst_size ... 格納先のサイズ

戻り値
-3 ... 入力文字列の書式が正しくない
-2 ... 入力がutf-8でない
-1 ... 内部エラー
0 ... 成功
1以上 ... dstのサイズが小さすぎる。戻り値は必要なサイズ
*/


// src ... パースする文字列(utf-8)書式
int32_t parse_kif(const char* src, char* dst, int32_t dst_size);
int32_t parse_ki2(const char* src, char* dst, int32_t dst_size);
int32_t parse_csa(const char* src, char* dst, int32_t dst_size);
int32_t parse_jkf(const char* src, char* dst, int32_t dst_size);

// src ... jkf文字列(utf-8)
int32_t to_kif(const char* src, char* dst, int32_t dst_size);
int32_t to_ki2(const char* src, char* dst, int32_t dst_size);
int32_t to_csa(const char* src, char* dst, int32_t dst_size);


