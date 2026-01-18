package com.greenbulb.sonarpendemo2;

import android.content.Context;
import android.util.AttributeSet;
import android.view.MotionEvent;
import android.view.View;

import org.json.JSONObject;

public class TouchView extends View {
    private TouchCallBack thisInterface=null;

    public TouchView(Context context) {
        super(context);
    }

    public TouchView(Context context, AttributeSet attrs) {
        super(context,attrs);
    }

    public void assignInterface(TouchCallBack t) {
        thisInterface = t;
    }

    @Override
    public boolean onTouchEvent(MotionEvent evt) {
        if (thisInterface != null) {
            thisInterface.receiveMotionEvent(evt);
        }
        return true;
    }
}
