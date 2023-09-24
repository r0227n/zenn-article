---
title: "標準パッケージのみで画像アプリの拡大縮小を再現してみた"
emoji: "😊"
type: "tech" # tech: 技術記事 / idea: アイデア
topics: [Flutter]
published: true
---
![](https://storage.googleapis.com/zenn-user-upload/ae795cb097aa-20230925.gif)
[InteractiveViewer](https://api.flutter.dev/flutter/widgets/InteractiveViewer-class.html)はコンテンツを拡大縮小、スクロールできるWidgetです。
標準ではパンでしかコンテンツを操作できないので、タップでも拡大縮小できるようにカスタマイズする方法をご紹介します。
(画像アプリでよくある拡大縮小の操作を実装していきます！)

# ロジックの解説
拡大縮小やスクロールなどは[Matrix4](https://api.flutter.dev/flutter/vector_math/Matrix4-class.html)を使い
- 行列の乗算
- 逆行列の計算
- ベクトルの変換

など、4x4の行列を操作し実装しています。
(Flutterは行列関連のパッケージを標準で多く実装しており、開発者が直接行列を操作することは少ないです。)

## InteractiveViewerの行列操作の状態管理
[TransformationController](https://api.flutter.dev/flutter/widgets/TransformationController-class.html)(`Matrix4`を`ValueNotifier`でラップしたクラス)でStateを管理しています。

## タップで拡大縮小する仕組み
[GestureDetector](https://api.flutter.dev/flutter/widgets/GestureDetector-class.html)(ジェスチャー操作を検出するWidget)を使い、[Matrix4Tween](https://api.flutter.dev/flutter/widgets/Matrix4Tween-class.html)でタップ前後の`Matrix4`をを補完します。

# 実装
https://github.com/r0227n/zenn-article/blob/develop/samples/pinch_zoom/lib/pinch_zoom.dart
:::details Widget
```dart
class PinchZoom extends StatefulWidget {
  const PinchZoom({
    this.controller,
    required this.child,
    this.backgroudColor,
    this.minScale = 1.0,
    this.maxScale = 10.0,
    this.scale = 3.0,
    this.animationController,
    this.curve = Curves.easeOut,
    this.onTap,
    this.onTapDown,
    this.onDoubleTap,
    super.key,
  });

  final PinchZoomController? controller;
  final Widget child;
  final Color? backgroudColor;
  final double minScale;
  final double maxScale;

  /// Scale of child
  final double scale;

  final AnimationController? animationController;

  /// An parametric animation easing curve, i.e. a mapping of the unit interval to
  /// the unit interval.
  final Curve curve;

  final GestureTapCallback? onTap;
  final GestureTapDownCallback? onTapDown;
  final GestureTapCallback? onDoubleTap;

  @override
  State<PinchZoom> createState() => _PinchZoomState();
}

class _PinchZoomState extends State<PinchZoom> with SingleTickerProviderStateMixin {
  late final PinchZoomController _controller;
  TapDownDetails? _doubleTapDetails;

  late final AnimationController _animationController;

  @override
  void initState() {
    super.initState();
    _animationController = widget.animationController ??
        AnimationController(
          vsync: this, // the SingleTickerProviderStateMixin
          duration: const Duration(milliseconds: 400),
        );

    _controller = widget.controller ??
        PinchZoomController(
          animationController: _animationController,
        );
  }

  @override
  void dispose() {
    if (widget.animationController == null) {
      _animationController.dispose();
    }
    if (widget.controller == null) {
      _controller.dispose();
    }
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Material(
      color: widget.backgroudColor,
      child: GestureDetector(
        onTap: widget.onTap,
        onTapDown: (details) {
          _doubleTapDetails = details;

          WidgetsBinding.instance.addPersistentFrameCallback((_) {
            if (widget.onTapDown is GestureTapDownCallback && mounted) {
              widget.onTapDown!(details);
            }
          });
        },
        onDoubleTap: () {
          if (widget.onDoubleTap is GestureTapCallback && mounted) {
            widget.onDoubleTap!();
          }

          if (_doubleTapDetails?.localPosition != null) {
            _controller.transform(
              offset: _doubleTapDetails!.localPosition,
              scale: widget.scale,
              curve: widget.curve,
            );
          }
        },
        child: InteractiveViewer(
          transformationController: _controller,
          minScale: widget.minScale,
          maxScale: widget.maxScale,
          onInteractionStart: (details) => _controller.onInteractionStart(details),
          child: widget.child,
        ),
      ),
    );
  }
}
```
:::
:::details Controller
```dart
class PinchZoomController extends TransformationController {
  PinchZoomController({
    required this.animationController,
    Matrix4? value,
  }) : super(value ?? Matrix4.identity()) {
    animationController.addListener(() {
      if (animationState is Animation<Matrix4>) {
        this.value = animationState!.value;
      }
    });
  }

  /// Controller during animation of [PinchZoomController]
  final AnimationController animationController;

  /// Value of [Matrix4] that is animated.
  Animation<Matrix4>? animationState;
  Offset? scenePosition;

  /// Called when the user pan or scale gesture on the widget.
  /// Overwrite the value of [value] with [animationState]
  void _onAnimateReset() {
    value = animationState!.value;
    if (!animationController.isAnimating) {
      animationState!.removeListener(_onAnimateReset);
      animationState = null;
      animationController.reset();
    }
  }

  /// Transform [PinchZoomController] values.
  /// [offset] is the position of the user's finger on the screen.
  /// [scale] is the scale of the user's finger on the screen.
  /// [curve] is the animation easing curve.
  /// [fromAnimation] is the animation start value.
  TickerFuture transform({
    required Offset offset,
    required double scale,
    required Curve curve,
    double fromAnimation = 0,
  }) {
    scenePosition = toScene(offset);

    animationState = Matrix4Tween(
      begin: value,
      end: animationScale(scale),
    ).animate(
      CurveTween(curve: curve).animate(animationController),
    );

    return animationController.forward(from: fromAnimation);
  }

  /// [PinhZoom]'s animation scale
  Matrix4? animationScale(double scale, {bool reverse = false, Offset? position}) {
    position ??= scenePosition;
    if (position == null) {
      return null;
    } else if (value != Matrix4.identity()) {
      return Matrix4.identity();
    } else if (reverse) {
      return Matrix4.identity()
        ..translate(position.dx * scale, position.dy * scale)
        ..scale(scale);
    }

    return Matrix4.identity()
      ..translate(-position.dx * scale, -position.dy * scale)
      ..scale(scale);
  }

  /// Reset [PinchZoomController] values.
  void reset() {
    animationController.reset();
    scenePosition = null;
    animationState = Matrix4Tween(
      begin: value,
      end: Matrix4.identity(),
    ).animate(animationController);
    animationState!.addListener(_onAnimateReset);
    animationController.forward();
  }

  /// Stop a running reset to home transform animation.
  void _animateResetStop() {
    animationController.stop();
    animationState?.removeListener(_onAnimateReset);
    animationState = null;
    animationController.reset();
  }

  /// Called when the user begins a pan or scale gesture on the widget.
  void onInteractionStart(ScaleStartDetails details) {
    // If the user tries to cause a transformation while the reset animation is
    // running, cancel the reset animation.
    if (animationController.status == AnimationStatus.forward) {
      _animateResetStop();
    }
  }
}
```
:::
## 状態管理
`TransformationController`を継承した`PinchZoomController`を作成し、`Matrix4`の状態とアニメーションを管理します。

## タップで拡大縮小する仕組み
現在地点と置換後地点の座標間を乗算し、Offsetオブジェクトを作成する[toScene](https://api.flutter.dev/flutter/widgets/TransformationController/toScene.html)を使い、タップした座標を`Matrix4`の座標に変換します。その後、[Matrix4Tween](https://api.flutter.dev/flutter/widgets/Matrix4Tween-class.html)を使い、タップ前後の`Matrix4`を補完しています。

# まとめ
今回は[InteractiveViewer](https://api.flutter.dev/flutter/widgets/InteractiveViewer-class.html)のサンプルを触った際、『タップで拡大縮小もできるのではないか？』と思い実装してみました。行列の計算やアニメーションが標準パッケージで幅広くサポートされており、予想以上に簡単に実装できました。
FlutterはUIだけでなく、UX面での実装もサポートされていて、このフレームワークがさらに好きになりました。
以上、[八木海莉](https://yagikairi.com/#/)の[さらば、私の星](https://www.youtube.com/watch?v=zHZk5WMz530)を聴きながら書いたRyo24でした。