Digital root

Дигиталния корен на едно число може да получим като съберем цифрите му, после съберем цифрите на резултата му и така нататък, докато получим една-единствена цифра. Примерно, за числото `345`:

```
3 + 4 + 5 = 12
1 + 2 = 3
```

И така, дигиталния корен на `345` е цифрата `3`. Или поне в десетична бройна система. В шестнадесетична, дигиталния корен би бил `c`. Друг пример в шестнадесетична система, `7b`:

```
7 + b = 12 (7 + 11 = 18 в десетични сметки)
1 + 2 = 3
```

Корена на `7b` е `3`, поне в тази бройна система.

Искаме от вас да имплементирате няколко функции, които да изчислят дигиталния корен в конкретни бройни системи. Нужно е да се справяте само с неотрицателни числа. Входа е низ, който бива интерпретиран като число в съответната бройна система. Изхода е число (опаковано в Option), което би трябвало да е една "цифра" в тази бройна система.

Ако подадем низ, който е невалиден за дадената бройна система, очакваме функциите да върнат None.

Някои неща, за които не е нужно да се тревожите твърде много:

    Няма да подаваме низове, чиято сума на цифрите да надхвърли капацитета на u32.
    Няма да подаваме празни низове или низове с интервали или специални символи (низовете тук са просто механизъм за опростяване на входа).
    Няма да подаваме низове, започващи с +, въпреки, че са валидни неотрицателни числа.
    За низовете в шестнадесетична бройна система, ще подаваме само малки букви, тоест "7b" но не "7B".
