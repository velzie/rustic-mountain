package me.coolelectronics.rusticceleste;

import android.annotation.SuppressLint;
import android.app.Activity;
import android.content.Context;
import android.graphics.Canvas;
import android.graphics.Color;
import android.graphics.Paint;
import android.graphics.Rect;
import android.util.DisplayMetrics;
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
        outerPaint.setColor(getResources().getColor(R.color.purple_200));

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



    @Override
    protected void onDraw(Canvas canvas) {
        super.onDraw(canvas);

        int width = getWidth();
        int height = getHeight();
        int largestsize = height/128;

        // below four lines of code is use to add
        // back color to our screen which is green
        canvas.drawPaint(outerPaint);

        // on below line we are setting color to our paint.
        otherPaint.setColor(Color.WHITE);

        // on below line we are setting style to out paint.
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

        byte[] screen = RusticFFI.tick_screen();

        int scale = largestsize;


        int startx = width/2 - 128*scale/2;
        int starty = height/2 - 128*scale/2;

        for (int i=0; i<128*128;i++){
            int x = i % 128 * scale;
            int y = i / 128 * scale;

//            Log.e("t", String.valueOf(screen.length));
            int[] c = pal[screen[i]];
            otherPaint.setColor(Color.rgb(c[0],c[1],c[2]));
            canvas.drawRect(
                    startx + x,
                    starty + y,
                    startx + x+scale,
                    starty + y+scale, otherPaint);
        }

        // on below line we are changing the color for our paint.
        otherPaint.setColor(getResources().getColor(R.color.teal_200));

        int celendx = startx + 128*scale;

        int buttonwidth = (width - celendx) / 3;
        int buttonstart = celendx + buttonwidth/2;

        int buttony = getBottom() - buttonwidth/2;

        jumpbutton = new Rect(buttonstart,buttony-buttonwidth,buttonstart + buttonwidth,buttony);
        buttonstart += buttonwidth*1.1;
//        buttony -= buttonwidth*1;
        dashbutton = new Rect(buttonstart,buttony-buttonwidth,buttonstart + buttonwidth,buttony);

        canvas.drawRect(jumpbutton,otherPaint);
        canvas.drawRect(dashbutton,otherPaint);
//
//        canvas.drawRect(joystickrect,otherPaint);


        invalidate();

    }
    boolean inRect(Rect r,int x,int y){
        return x>r.left && x<r.right && y>r.top && y<r.bottom;
    }
    @Override
    public boolean onTouchEvent(MotionEvent ev) {
        // Let the ScaleGestureDetector inspect all events.
//        mScaleDetector.onTouchEvent(ev);

        final int action = ev.getAction();
        Log.e("t", String.valueOf(action & 255));

        if ((ev.getAction() & MotionEvent.ACTION_MASK) == MotionEvent.ACTION_DOWN) {
            if (inRect(jumpbutton, (int) ev.getX(), (int) ev.getY()))
                RusticFFI.set_btn(4,true);
            if (inRect(dashbutton, (int) ev.getX(), (int) ev.getY()))
                RusticFFI.set_btn(5,true);
        }
        if ((ev.getAction() & MotionEvent.ACTION_MASK) == MotionEvent.ACTION_UP) {
            RusticFFI.set_btn(4,false);
            RusticFFI.set_btn(5,false);
        }
//        if (ev.getX() > 300){
//            RusticFFI.set_btn(1,true);
//
//        }else{
//            RusticFFI.set_btn(1,false);
//        }

        return true;
    }
}