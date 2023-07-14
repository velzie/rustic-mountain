package me.coolelectronics.rusticceleste;

import android.annotation.SuppressLint;
import android.content.Context;
import android.graphics.Canvas;
import android.graphics.Color;
import android.graphics.Paint;
import android.graphics.Rect;
import android.os.SystemClock;
import android.util.Log;
import android.view.MotionEvent;
import android.view.View;

public class PaintView extends View {

    // below we are creating variables for our paint
    Paint otherPaint, outerPaint, textPaint;

    // and a floating variable for our left arc.
    Rect jumpbutton;
    Rect dashbutton;

    Rect joystickrect;
    int joyptr = -1;

    long lasttime   =System.currentTimeMillis();
    @SuppressLint("ResourceAsColor")
    public PaintView(Context context) {
        super(context);

        // on below line we are initializing our paint variable for our text
        textPaint = new Paint(Paint.LINEAR_TEXT_FLAG | Paint.ANTI_ALIAS_FLAG);

        // on below line we are setting color to it.
        textPaint.setColor(Color.WHITE);

        // on below line we are setting text size to it.
        // In Paint we have to add text size using px so
        // we have created a method where we are converting dp to pixels.
//        textPaint.setTextSize(pxFromDp(context, 24));

        // on below line we are initializing our outer paint
        outerPaint = new Paint();

        // on below line we are setting style to our paint.
        outerPaint.setStyle(Paint.Style.FILL);

        // on below line we are setting color to it.
        outerPaint.setColor(getResources().getColor(R.color.black));

//        // on below line we are creating a display metrics
//        DisplayMetrics displayMetrics = new DisplayMetrics();
//
//        // on below line we are getting display metrics.
//        ((Activity) getContext()).getWindowManager()
//                .getDefaultDisplay()
//                .getMetrics(displayMetrics);
//        arcLeft = pxFromDp(context, 20);
//
//
//        // on below line we are assigning
//        // the value to the arc left.
//        // on below line we are creating
        // a new variable for our paint
        otherPaint = new Paint();
    }

//
//    // below method is use to generate px from DP.
//    public static float pxFromDp(final Context context, final float dp) {
//        return dp * context.getResources().getDisplayMetrics().density;
//    }


    final long target_fps = 30;
    final long interval = 1000/target_fps;

    int joyx = -1;
    int joyy = -1;


    @Override
    protected void onDraw(Canvas canvas) {
        super.onDraw(canvas);


        long time = System.currentTimeMillis();
        long elapsedmillis = time-lasttime;


        // cap fps to 30
        while (elapsedmillis < interval){
            SystemClock.sleep(interval-elapsedmillis);
            time = System.currentTimeMillis();
            elapsedmillis = time-lasttime;
        }
        lasttime = time;
        RusticFFI.game_tick();

        int width = getWidth();
        int height = getHeight();
        int largestsize = height/128;


        canvas.drawPaint(outerPaint);

        otherPaint.setStyle(Paint.Style.FILL);

        int[][] pal = {
                {0, 0, 0},
        {29, 43, 83},
        {126, 37, 83},
        {0, 135, 81},
        {171, 82, 54},
        {95, 87, 79},
        {194, 195, 199},
        {255, 241, 232},
        {255, 0, 77},
        {255, 163, 0},
        {255, 236, 85},
        {0, 228, 54},
        {41, 173, 255},
        {131, 118, 156},
        {255, 119, 168},
        {255, 204, 170},
        };


        byte[] screen = RusticFFI.draw_screen();


        int scale = largestsize;


        int startx = width/2 - 128*scale/2;
        int starty = height/2 - 128*scale/2;

        for (int i=0; i<128*128;i++){
            int x = i % 128 * scale;
            int y = i / 128 * scale;

            int[] c = pal[screen[i]];
            otherPaint.setColor(Color.rgb(c[0],c[1],c[2]));
            canvas.drawRect(
                    startx + x,
                    starty + y,
                    startx + x+scale,
                    starty + y+scale, otherPaint);
        }

        otherPaint.setColor(Color.rgb(45,45,45));

        int celendx = startx + 128*scale;

        int buttonwidth = (width - celendx) / 3;
        int buttonstart = celendx + buttonwidth/2;

        int buttony = getBottom() - buttonwidth/2;

        jumpbutton = new Rect(buttonstart,buttony-buttonwidth,buttonstart + buttonwidth,buttony);
        buttonstart += buttonwidth*1.1;
//        buttony -= buttonwidth*1;
        dashbutton = new Rect(buttonstart,buttony-buttonwidth,buttonstart + buttonwidth,buttony);



        int joystart = startx - (int)(buttonwidth * 2.5);

        joystickrect = new Rect(joystart,buttony-(buttonwidth*2),joystart + buttonwidth*2,buttony);


        canvas.drawRect(jumpbutton,otherPaint);
        canvas.drawRect(dashbutton,otherPaint);
        canvas.drawRect(joystickrect,otherPaint);
        if (joyx != -1){
            otherPaint.setColor(Color.rgb(65,65,65));
            canvas.drawRect(joyx-buttonwidth/4,joyy-buttonwidth/4,joyx+buttonwidth/2,joyy+buttonwidth/2,otherPaint);
        }


        invalidate();

    }
    boolean inRect(Rect r,int x,int y){
        return x>r.left && x<r.right && y>r.top && y<r.bottom;
    }
    @Override
    public boolean onTouchEvent(MotionEvent ev) {
        // Let the ScaleGestureDetector inspect all events.
//        mScaleDetector.onTouchEvent(ev);

        final int action = (ev.getAction() & MotionEvent.ACTION_MASK);

        if (action == MotionEvent.ACTION_DOWN || action == MotionEvent.ACTION_POINTER_DOWN || action == MotionEvent.ACTION_BUTTON_PRESS || action == MotionEvent.ACTION_MOVE || action == MotionEvent.ACTION_POINTER_UP) {

                int x = (int) ev.getX(ev.getActionIndex());
                int y = (int) ev.getY(ev.getActionIndex());
                if (inRect(jumpbutton, x, y)) {
                    RusticFFI.set_btn(5, true);
                }
                if (inRect(dashbutton, x, y))
                    RusticFFI.set_btn(4,true);
                if (inRect(joystickrect,x,y) || ev.getPointerId(ev.getActionIndex()) == joyptr){
                    joyx = x;
                    joyy = y;
                    joyptr = ev.getPointerId(ev.getActionIndex());
                    int midx = joystickrect.centerX();
                    int w = joystickrect.width();
                    int midy = joystickrect.centerY();
                    int h = joystickrect.height();
                    if (x>midx + w/6){
                        RusticFFI.set_btn(1,true);
                    }else{
                        RusticFFI.set_btn(1,false);
                    }
                    if (x<midx - w/6){
                        RusticFFI.set_btn(0,true);
                    }else{
                        RusticFFI.set_btn(0,false);
                    }
                    if (y>midy + h/6){
                        RusticFFI.set_btn(3,true);
                    }else{
                        RusticFFI.set_btn(3,false);
                    }
                    if (y<midy - h/6){
                        RusticFFI.set_btn(2,true);
                    }else{
                        RusticFFI.set_btn(2,false);
                    }
                }


        }
        if (action == MotionEvent.ACTION_UP || action == MotionEvent.ACTION_POINTER_UP) {
//            Log.e(":::", String.valueOf(ev.findPointerIndex(0)));
            if (ev.getPointerId(ev.getActionIndex()) == joyptr) {
                joyptr = -1;
                joyx = -1;
                RusticFFI.set_btn(0,false);
                RusticFFI.set_btn(1,false);
                RusticFFI.set_btn(2,false);
                RusticFFI.set_btn(3,false);
            }else{

                RusticFFI.set_btn(4,false);
                RusticFFI.set_btn(5,false);
            }
        }


        return true;
    }
}