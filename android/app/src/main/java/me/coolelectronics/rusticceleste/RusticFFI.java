package me.coolelectronics.rusticceleste;

public class RusticFFI {
    static {
        System.loadLibrary("rusticceleste");
    }


    native static void init();
    native static void start();


    native static byte[] tick_screen();
    native static void set_btn(int btn,boolean val);

}
