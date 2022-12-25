# SLR-автомат с переменным lookahead

## Задача: 
По заданному числу N и грамматике построить SLR-автомат. Если в грамматике встречаются конфликты, вместо SLR-автомата построить LR(0)-автомат и проверить каждый конфликт на разрешимость при всех значениях lookahead <= N.

Конфликтом считается состояние автомата, в котором невозможно точно определить дальнейший порядок разбора из-за возможной свертки:

```
[A] -> a. //Вернуться по А?
[B] -> b[A]a. //Вернуться по B?
```

Или

```
[A] -> a. //Вернуться по A?
[B] -> b[A]a.c // Продолжить по B?
```

Конфликт считается разрешенным при lookahead = k, если:

- В первом случае: FOLLOW<sub>k</sub>(A) не имеет пересечений с FOLLOW<sub>k</sub>(B);
- Во втором случае: FOLLOW<sub>k</sub>(A) не имеет пересечений с FIRST<sub>k</sub>(FIRST<sub>k</sub>(*Текущего состояния B*) ++ FOLLOW<sub>k</sub>(B)).

Конфлит считается подвешенным при lookahead = k, если:

- В первом случае: FOLLOW<sub>k</sub>(A) полностью совпадает с FOLLOW<sub>k</sub>(B);
- Во втором случае: FOLLOW<sub>k</sub>(A) полностью совпадает с FIRST<sub>k</sub>(FIRST<sub>k</sub>(*Текущего состояния B*) ++ FOLLOW<sub>k</sub>(B)).

Конфликт считается неразрешенным при lookahead = k, если не выполнены оба соответствующих условия выше.

## Входные данные: 
Натуральное число N и грамматика, построенная по следующим правилам:

- [Grammar] ::= [Rule]<sup>+</sup>
- [Rule] ::= [Nterm] "->" ([Term] | [Nterm])\*( "|" ([Term] | [Nterm])<sup>+</sup>)\*
- [Term] ::= [A-z]  | [0-9]
- [Nterm] ::= "[" [A-z]<sup>+</sup>[0-9]\* "]"

При этом объекты в кавычках явялются терминальными символами. Расстановка пробелов и табуляций может быть произвольна. Правила разделяются символом новой строки \n или набором символов \r\n. Должен присутствовать начальный нетерминал [S] (иначе в автомате будет всего лишь два состояния: `[♥] -> .[S]` и `[♥] -> [S].`). Ввод завершается пустой строкой.

### Пример правильных входных данных:
```
3
[S] -> [A][B]
[A] -> a[A] | b
[B] -> b[B]
[B] -> c
[Пустая строка]
```
### Примеры неправильных входных данных:
Отсутствует N:
```
[S] -> [A]b
[A] -> a[A] | c
[Пустая строка]
```
Отсутствует начальный нетерминал (S будет считаться терминальным символом):
```
3
S -> [A]
[A] -> a[A] | b
[Пустая строка]
```
Незакрытая скобка:
```
3
[S -> a[A]
[A] -> b
[Пустая строка]
```
## Выходные данные:
- Сообщение о синтаксической ошибке

Или

- https://dreampuf.github.io/GraphvizOnline/ - ссылка на парсер и визуализатор языка DOT;
- Описание направленного графа в языке DOT, являющегося схемой автомата; 
- Информация о конфликтах (участники, разрешились ли), дополнительное сообщение о полной разрешимости (если она есть).

### Пример выходных данных:
```
https://dreampuf.github.io/GraphvizOnline/
digraph {
1 [shape = "rectangle" label = "[♥] -> .[S]
[S] -> .[A][B]
[A] -> .a[A] | .b"]
2 [shape = "rectangle" peripheries = 2 label = "[♥] -> [S]."]
3 [shape = "rectangle" label = "[S] -> [A].[B]
[B] -> .b[B] | .c"]
4 [shape = "rectangle" label = "[S] -> [A][B]."]
5 [shape = "rectangle" label = "[B] -> b.[B]
[B] -> .b[B] | .c"]
6 [shape = "rectangle" label = "[B] -> b[B]."]
7 [shape = "rectangle" label = "[B] -> c."]
8 [shape = "rectangle" label = "[A] -> a.[A]
[A] -> .a[A] | .b"]
9 [shape = "rectangle" label = "[A] -> a[A]."]
10 [shape = "rectangle" label = "[A] -> b."]
1 -> 2 [label = "S"]
1 -> 3 [label = "A"]
3 -> 4 [label = "B"]
3 -> 5 [label = "b"]
5 -> 6 [label = "B"]
5 -> 5 [label = "b"]
5 -> 7 [label = "c"]
3 -> 7 [label = "c"]
1 -> 8 [label = "a"]
8 -> 9 [label = "A"]
8 -> 8 [label = "a"]
8 -> 10 [label = "b"]
1 -> 10 [label = "b"]
}
Конфликтов нет (или все были разрешены)
```

## Пример работы программы:

Входные данные:
```
3
[S] -> [A][B]
[A] -> id | idiom | idbomb | idcomb
[B] -> bomb | comb
[Пустая строка]
```
Результат работы:
```
https://dreampuf.github.io/GraphvizOnline/
digraph {
1 [shape = "rectangle" label = "[♥] -> .[S]
[S] -> .[A][B]
[A] -> .id | .idiom | .idbomb | .idcomb"]
2 [shape = "rectangle" peripheries = 2 label = "[♥] -> [S]."]
3 [shape = "rectangle" label = "[S] -> [A].[B]
[B] -> .bomb | .comb"]
4 [shape = "rectangle" label = "[S] -> [A][B]."]
5 [shape = "rectangle" label = "[B] -> b.omb"]
6 [shape = "rectangle" label = "[B] -> bo.mb"]
7 [shape = "rectangle" label = "[B] -> bom.b"]
8 [shape = "rectangle" label = "[B] -> bomb."]
9 [shape = "rectangle" label = "[B] -> c.omb"]
10 [shape = "rectangle" label = "[B] -> co.mb"]
11 [shape = "rectangle" label = "[B] -> com.b"]
12 [shape = "rectangle" label = "[B] -> comb."]
13 [shape = "rectangle" label = "[A] -> i.d | i.diom | i.dbomb | i.dcomb"]
14 [shape = "rectangle" label = "[A] -> id. | id.iom | id.bomb | id.comb"]
15 [shape = "rectangle" label = "[A] -> idi.om"]
16 [shape = "rectangle" label = "[A] -> idio.m"]
17 [shape = "rectangle" label = "[A] -> idiom."]
18 [shape = "rectangle" label = "[A] -> idb.omb"]
19 [shape = "rectangle" label = "[A] -> idbo.mb"]
20 [shape = "rectangle" label = "[A] -> idbom.b"]
21 [shape = "rectangle" label = "[A] -> idbomb."]
22 [shape = "rectangle" label = "[A] -> idc.omb"]
23 [shape = "rectangle" label = "[A] -> idco.mb"]
24 [shape = "rectangle" label = "[A] -> idcom.b"]
25 [shape = "rectangle" label = "[A] -> idcomb."]
1 -> 2 [label = "S"]
1 -> 3 [label = "A"]
3 -> 4 [label = "B"]
3 -> 5 [label = "b"]
5 -> 6 [label = "o"]
6 -> 7 [label = "m"]
7 -> 8 [label = "b"]
3 -> 9 [label = "c"]
9 -> 10 [label = "o"]
10 -> 11 [label = "m"]
11 -> 12 [label = "b"]
1 -> 13 [label = "i"]
13 -> 14 [label = "d"]
14 -> 15 [label = "i"]
15 -> 16 [label = "o"]
16 -> 17 [label = "m"]
14 -> 18 [label = "b"]
18 -> 19 [label = "o"]
19 -> 20 [label = "m"]
20 -> 21 [label = "b"]
14 -> 22 [label = "c"]
22 -> 23 [label = "o"]
23 -> 24 [label = "m"]
24 -> 25 [label = "b"]
}
Конфликт между
[A] -> id.
[A] -> id.iom
[A] -> id.bomb
[A] -> id.comb
не был разрешён при k = 3
```

## Запуск программы:

Python:

`python3 ./SLR.py`

Rust (исходный код в main.rs):

`./SLR`


