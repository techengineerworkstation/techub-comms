import 'package:flutter/material.dart';

class AppTheme {
  // Metallic Beige
  static const Color beige50 = Color(0xFFfdf8f0);
  static const Color beige100 = Color(0xFFf5ead6);
  static const Color beige200 = Color(0xFFecdcb8);
  static const Color beige300 = Color(0xFFdfc89a);
  static const Color beige400 = Color(0xFFd4b57e);
  static const Color beige500 = Color(0xFFc4a06a);

  // Tanned Turquoise
  static const Color teal50 = Color(0xFFe6f7f7);
  static const Color teal100 = Color(0xFFb3e8e8);
  static const Color teal200 = Color(0xFF80d9d9);
  static const Color teal300 = Color(0xFF4dcaca);
  static const Color teal400 = Color(0xFF26bfbf);
  static const Color teal500 = Color(0xFF009999);
  static const Color teal600 = Color(0xFF007a7a);
  static const Color teal700 = Color(0xFF005c5c);

  // Metallic Accents
  static const Color metallicLight = Color(0xFFd4cfc7);
  static const Color metallicMid = Color(0xFFb8b0a2);
  static const Color metallicDark = Color(0xFF8a8070);
  static const Color metallicGold = Color(0xFFc9a84c);

  static const LinearGradient turquoiseGradient = LinearGradient(
    begin: Alignment.topLeft,
    end: Alignment.bottomRight,
    colors: [teal500, teal600, teal700],
  );

  static const LinearGradient metallicGradient = LinearGradient(
    begin: Alignment.topLeft,
    end: Alignment.bottomRight,
    colors: [beige100, beige300, beige500],
  );

  static const LinearGradient heroGradient = LinearGradient(
    begin: Alignment.topLeft,
    end: Alignment.bottomRight,
    colors: [Color(0xFF005c5c), teal500, Color(0xFF26bfbf)],
  );

  static ThemeData get metallicTurquoise {
    return ThemeData(
      useMaterial3: true,
      colorScheme: ColorScheme.fromSeed(
        seedColor: teal500,
        primary: teal500,
        secondary: beige500,
        surface: beige50,
        brightness: Brightness.light,
      ),
      scaffoldBackgroundColor: beige50,
      fontFamily: 'Roboto',
      appBarTheme: AppBarTheme(
        backgroundColor: Colors.white,
        foregroundColor: Colors.grey[900],
        elevation: 0,
        shadowColor: Colors.black.withOpacity(0.05),
        surfaceTintColor: Colors.transparent,
      ),
      cardTheme: CardThemeData(
        color: Colors.white,
        elevation: 0,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(12),
          side: BorderSide(color: beige100),
        ),
        shadowColor: Colors.black.withOpacity(0.08),
      ),
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          backgroundColor: teal500,
          foregroundColor: Colors.white,
          elevation: 2,
          shadowColor: teal500.withOpacity(0.4),
          padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 14),
          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
        ),
      ),
      outlinedButtonTheme: OutlinedButtonThemeData(
        style: OutlinedButton.styleFrom(
          foregroundColor: teal700,
          side: BorderSide(color: beige200),
          padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 14),
          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
        ),
      ),
      inputDecorationTheme: InputDecorationTheme(
        filled: true,
        fillColor: Colors.white,
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(10),
          borderSide: BorderSide(color: beige200),
        ),
        enabledBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(10),
          borderSide: BorderSide(color: beige200),
        ),
        focusedBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(10),
          borderSide: const BorderSide(color: teal400, width: 2),
        ),
        contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 14),
      ),
    );
  }
}
