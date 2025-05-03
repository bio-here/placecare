[![Version](https://img.shields.io/badge/version-0.1.1-green.svg)]()
[![GitHub](https://img.shields.io/badge/github-bio--here%2Fplacecare-blue.svg)](https://github.com/bio-here/placecare)
[![Build Status](https://travis-ci.org/bio-here/placecare.svg?branch=master)](https://travis-ci.org/bio-here/placecare)
[![Crates.io](https://img.shields.io/crates/v/placecare.svg)](https://crates.io/crates/placecare)
[![Documentation](https://docs.rs/placecare/badge.svg)](https://docs.rs/placecare)
[![License](https://img.shields.io/crates/l/MIT.svg)]()

阅读其他语言版本的文档：
- [English](README.md)

# PLACE-CARE

placecare 是一个使用 PLACE数据库 基于字符串搜索算法预测顺式作用元件的工具。

使用 placecare，你可以：

1. 上传序列文件搜索顺式作用元件。

2. 通过 PLACE 数据库的 id 和 ac 快速获取相关信息
（数据来自 PLACE 官网提供的 place.seq 文件）


# 安装

如果你的电脑上包含Rust工具链，你可以使用如下命令安装我们的命令行程序：

```shell
cargo install placecare
```

如果你并没有安装Rust工具链，你也可以在 GitHub的Release中 直接下载我们编译后的二进制文件：
- [Release](https://bio-here.github.io/placecare/release)


如果你要使用我们的库，只需要：
```shell
cargo add placecare

```

# 使用

这里介绍了我们的库如何使用。
placehere的核心功能编写在 `place_search` 模块中，I/O操作编写在 `io` 模块，
`place_desc` 模块是对PLACE数据的描述文件。

## 搜索元件

我们提供了多种输入序列的方式，如下所示：
```rust
use placehere::io::RecordDesc;

let input = vec![RecordDesc::new("Gh_01", "TTATAGACTCGATGGCCGCGCGG")];
let input = RecordDesc::from_file("./input.fasta");
let input = RecordDesc::from_string("\
>Gh_01
ATATCCGGATGGCATGCTGATC
");
let input = RecordDesc::from_records(bio::io::fasta::Reader::new("./input.fasta"));

let mut f = File::open("input.txt").unwrap();
let input = RecordDesc::from_reader(f);
```

然后我们可以进行搜索：
```rust
use placecare::place_search::Search;

// 搜索单个元件
let result = Search::search_element(input).unwrap();

// 搜索多个元件
let result = Search::search_elements(input).unwrap();
```

可以查看 `place_desc` 模块中的定义了解输出信息。

## 查询元件信息

我们可以使用以下方法查询PLACE数据库中的元件信息。
```rust
use biohere_placecare::search::Search;

The function will return a vector of Option<SeqDesc>
// for which is a result of the input sequence.
let e1: Vec<Option<SeqDesc>> = query_elements_by_id(&vec!["TATABOX1", "TATABOX2"]);
let e2: Vec<Option<SeqDesc>> = query_elements_by_ac(&vec!["S000023", "S000260"]);
```

# 提示

## IUPAC模糊碱基
PLACE 数据库中使用了 IUPAC 模糊碱基符号 （[WikiPedia](https://en.wikipedia.org/wiki/Nucleic_acid_notation)）来表示多种可能的碱基。


# 协议
placecare 是一个开源项目，遵循 MIT 许可证。你可以自由使用、修改和分发这个软件，但请保留原作者的版权声明和许可证信息。
