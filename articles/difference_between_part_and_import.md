---
title: "[Dart]partã¨importã®éã"
emoji: "ð"
type: "tech" # tech: æè¡è¨äº / idea: ã¢ã¤ãã¢
topics: ["dart", "flutter"]
published: true
---

# ã¯ããã«
ããã«ã¡ã¯ã[Ryo24](https://twitter.com/r0227n_)ã§ãã  
Dartã¯`part`ã¨`import`ã®2ç¨®é¡ã§ãã¡ã¤ã«ãåç§ãããã¨ãã§ãã¾ãã  
æ¬è¨äºã¯2ã¤ã®éãã¨ä½¿ãåãã«é¢ãã¦èªåç¨ã®ã¡ã¢ã¨ãã¦æ¸ãã¾ããã

## partã¨importã®éãã¯ä½ï¼
## partã¨ã¯
`part`ã¯`part of`ã¨ã»ããã§ä½¿ãããã­ã°ã©ã ãåå²ãã¾ãã
```php: one.dart
part 'two.dart';

test();  // print 'test'
```

```php: two.dart
part of 'one.dart';

void test() {
    print('test');
}
```

`one.dart`(part)ã®æ¡å¼µã¨ãã¦`two.dart`(part of)ãããã¾ãã  

## importã¨ã¯
`import`ã¯ãã¡ã¤ã«ãåç§ãã¾ãã
```php: one.dart
import 'two.dart';

test();  // print 'test'
```

```php: two.dart
void test() {
    print('test');
}
```

## éãã¯ä½ï¼
ä¸è¦ã`part`ã¨`import`ã®éããããã¾ãã...
ãã®2ã¤ã®éãã¯ã
> In Dart, private members are accessible within the same library. With import you import a library and can access only its public members. With part/part of you can split one library into several files and private members are accessible for all code within these files.  
> å¼ç¨å: [When to use part/part of versus import/export in Dart?](https://stackoverflow.com/questions/27763378/when-to-use-part-part-of-versus-import-export-in-dart#:~:text=In%20Dart%2C%20private%20members%20are,all%20code%20within%20these%20files.)

ã¤ã¾ãã
- `part` : **public**ã¨**private**ã©ã¡ããèª­ã¿è¾¼ã
- `import` : **public**ããèª­ã¿è¾¼ã¾ãªã

ä¸è¨ã®éãããããåç§åå®¹ãå®å¨ã«ç°ãªããã¨ããããã¾ãã

# çµè«
åºæ¬çã«`import`ãä½¿ãã¾ãããã
## partãä½¿ãã¿ã¤ãã³ã°
`freezed`ãã¯ããã¨ãããã³ã¼ããèªåçæããå®è£ã§ä½¿ãã
:::message
ã©ã¤ãã©ãªä½ææãpartã§ã¯ãªãããã¢ããªãä½æãimportãããã¨ãæ¨å¥¨ããã¦ãã¾ãã
:::
## importãä½¿ãã¿ã¤ãã³ã°
ã³ã³ãã¼ãã³ããæ´»ç¨ããæ©è½ãè¿½å ããã¨ãã«ä½¿ã(= åºæ¬çã«ããã¤ãä½¿ã)ã

# åèæç®
https://stackoverflow.com/questions/27763378/when-to-use-part-part-of-versus-import-export-in-dart

https://dart.dev/guides/libraries/create-library-packages#organizing-a-library-package
