import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:techub_mobile/main.dart';

void main() {
  testWidgets('App renders correctly', (WidgetTester tester) async {
    await tester.pumpWidget(const TechubCommsApp());
    expect(find.text('Techub Comms'), findsOneWidget);
  });
}
