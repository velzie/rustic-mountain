package me.coolelectronics.rusticceleste;

public class RusticFFI {
    static {
        System.loadLibrary("rusticceleste");
    }


    native static void init();
    native static void start();


    native static byte[] draw_screen();
    native static void game_tick();
    native static void set_btn(int btn,boolean val);

}
