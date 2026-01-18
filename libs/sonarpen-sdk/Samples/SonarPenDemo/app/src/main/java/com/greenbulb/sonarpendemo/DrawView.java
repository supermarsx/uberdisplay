package com.greenbulb.sonarpendemo;

import android.content.Context;
import android.graphics.Bitmap;
import android.graphics.Canvas;
import android.graphics.Color;
import android.graphics.Paint;
import android.graphics.Point;
import android.os.Environment;
import android.util.AttributeSet;
import android.view.MotionEvent;
import android.view.View;

import org.json.JSONObject;

import java.io.File;
import java.io.FileOutputStream;
import java.util.Date;
import java.text.SimpleDateFormat;
import java.util.HashMap;
import java.util.Hashtable;

public class DrawView extends View {
    private float dx,dy;
    private float rdp;
    private boolean bPenOn=false;
    private Bitmap cacheBitmap=null;
    private int currentPenIdx=0;
    private float penWidth=100.0f;
    private boolean bWriteLog=false;     // generate log file
    private MainActivityCallback delegate=null;

    private HashMap<String,penPoints> lastPoints = new HashMap<String,penPoints>();

    private class penPoints {
        float dx,dy;
        float rdp;
    }

    public void setDelegate(MainActivityCallback d) {
        delegate = d;
    }

    public DrawView(Context context) {
        super(context);
    }

    public DrawView(Context context, AttributeSet attrs) {
        super(context,attrs);
    }

    public void clearScreen() {
        // Clear Drawing Screen
        cacheBitmap = newDrawingPaper();
        lastPoints.clear();
        invalidate();
    }

    private Bitmap newDrawingPaper() {
        // Create backup Drawing Screen Bitmap
        Bitmap bmp = Bitmap.createBitmap(getWidth(),getHeight(), Bitmap.Config.ARGB_8888);
        Paint p = new Paint();
        p.setAntiAlias(true);
        p.setColor(Color.WHITE);
        p.setStyle(Paint.Style.FILL_AND_STROKE);
        Canvas c = new Canvas(bmp);
        c.drawRect(0,0,getWidth(),getHeight(),p);
        c.save();

//        penWidth = getWidth() / 3.0f;

        return bmp;
    }

    private void writeLog(String msg) {
        File extDir = Environment.getExternalStorageDirectory();
        if (extDir == null) {
            return;
        }
        String file_path = extDir.getAbsolutePath() + "/SonarPen";
        File dir = new File(file_path);
        if(!dir.exists()) try {
            dir.mkdirs();
        } catch (Exception e) {
            e.printStackTrace();
        }
        File f = new File(dir,"Log.txt");
        try {
            FileOutputStream fout = new FileOutputStream(f,true);
            SimpleDateFormat sdf = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss");
            String outstr =sdf.format(new Date())+" "+msg+"\r\n";
            fout.write(outstr.getBytes("UTF-8"));
            fout.close();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    public void setPenWidth(int size) {
        penWidth = size;
    }

    @Override
    public boolean onTouchEvent(MotionEvent evt) {
        // Handle Touch event
        if (evt.getAction() == MotionEvent.ACTION_MOVE) {
            MotionEvent.PointerCoords corrds = new MotionEvent.PointerCoords();
            MotionEvent.PointerProperties props = new MotionEvent.PointerProperties();
            for (int i=0;i<evt.getPointerCount();i++) {
                evt.getPointerProperties(i,props);
                evt.getPointerCoords(i,corrds);
                if (bWriteLog) writeLog(MotionEvent.actionToString(evt.getAction())+", "+((props.toolType == MotionEvent.TOOL_TYPE_STYLUS)?"STYLUS":"FINGER")+", pressure="+corrds.pressure+", size="+evt.getSize(i));
                int pid = evt.getPointerId(i);
                if (lastPoints.containsKey(Integer.toHexString(pid))) {
                    penPoints pts = lastPoints.get(Integer.toHexString(pid));
                    int historySize = evt.getHistorySize();
                    float px = pts.dx;
                    float py = pts.dy;
                    float rdp = pts.rdp;
                    float dvp = (corrds.pressure - rdp) / historySize;
                    if (historySize > 0) {
                        for (int j = 0; j < historySize; j++) {
                            float historicalX = evt.getHistoricalX(j);
                            float historicalY = evt.getHistoricalY(j);
                            if (props.toolType == MotionEvent.TOOL_TYPE_STYLUS)
                                drawLine(px, py, historicalX, historicalY, rdp + dvp * j, true);
                            else drawLine(px, py, historicalX, historicalY, 0.01f, false);
                            px = historicalX;
                            py = historicalY;
                        }
                    }
                    if (props.toolType == MotionEvent.TOOL_TYPE_STYLUS)
                        drawLine(px, py, corrds.x, corrds.y, corrds.pressure, true);
                    else drawLine(px, py, corrds.x, corrds.y, 0.01f, false);

                    pts.dx = corrds.x;
                    pts.dy = corrds.y;
                    pts.rdp = corrds.pressure;
                    lastPoints.put(Integer.toHexString(pid), pts);
                    addDrop(corrds.x, corrds.y, (props.toolType == MotionEvent.TOOL_TYPE_STYLUS)?corrds.pressure:0.01f, (props.toolType == MotionEvent.TOOL_TYPE_STYLUS));
                    invalidate();
                }
            }
        } else if ((evt.getActionMasked() == MotionEvent.ACTION_DOWN) || (evt.getActionMasked() == MotionEvent.ACTION_POINTER_DOWN)) {
            MotionEvent.PointerCoords corrds = new MotionEvent.PointerCoords();
            MotionEvent.PointerProperties props = new MotionEvent.PointerProperties();
            for (int i=0;i<evt.getPointerCount();i++) {
                evt.getPointerProperties(i,props);
                evt.getPointerCoords(i,corrds);
                int pid = evt.getPointerId(i);
                penPoints pts = new penPoints();
                pts.dx = corrds.x;
                pts.dy = corrds.y;
                pts.rdp = corrds.pressure;
                lastPoints.put(Integer.toHexString(pid),pts);
                addDrop(corrds.x, corrds.y, (props.toolType == MotionEvent.TOOL_TYPE_STYLUS)?corrds.pressure:0.01f, (props.toolType == MotionEvent.TOOL_TYPE_STYLUS));

                float px = pts.dx;
                float py = pts.dy;

                int historySize = evt.getHistorySize();
                if (historySize > 0) {
                    for (int j = 0; j < historySize; j++) {
                        float historicalX = evt.getHistoricalX(j);
                        float historicalY = evt.getHistoricalY(j);
                        if (props.toolType == MotionEvent.TOOL_TYPE_STYLUS)
                            drawLine(px, py, historicalX, historicalY, (props.toolType == MotionEvent.TOOL_TYPE_STYLUS)?corrds.pressure:0.01f, (props.toolType == MotionEvent.TOOL_TYPE_STYLUS));
                        else drawLine(px, py, historicalX, historicalY, (props.toolType == MotionEvent.TOOL_TYPE_STYLUS)?corrds.pressure:0.01f, (props.toolType == MotionEvent.TOOL_TYPE_STYLUS));
                        px = historicalX;
                        py = historicalY;
                    }
                }
                invalidate();
            }
        } else if (evt.getAction() == MotionEvent.ACTION_UP) {
            lastPoints.clear();
        } else if (evt.getAction() == MotionEvent.ACTION_POINTER_UP) {
            int id = evt.getPointerId(evt.getActionIndex());
            lastPoints.remove(Integer.toHexString(id));
        }
        if (delegate != null) delegate.touchevent_call();
        return true;
    }

    private void drawLine(float x0, float y0, float x, float y, float ddp,boolean bBlackColor) {
        Canvas c = new Canvas(cacheBitmap);
        Paint p = new Paint();
        p.setAntiAlias(true);
        p.setColor(bBlackColor?Color.BLACK:Color.RED);
        p.setStrokeWidth(ddp*(penWidth*getResources().getDisplayMetrics().density));
        p.setStrokeJoin(Paint.Join.ROUND);
        p.setStrokeCap(Paint.Cap.ROUND);
        p.setStyle(Paint.Style.FILL_AND_STROKE);
        c.drawLine(x0,y0,x,y,p);
        c.save();
    }

    private void addDrop(float x, float y,float ddp,boolean bBlackColor) {
        if (cacheBitmap == null) {
            cacheBitmap = newDrawingPaper();
        }
        dx = x;
        dy = y;
        rdp = ddp;

        Canvas c = new Canvas(cacheBitmap);
        Paint p = new Paint();
        p.setAntiAlias(true);
        p.setColor(bBlackColor?Color.BLACK:Color.RED);
        p.setStyle(Paint.Style.FILL_AND_STROKE);
        c.drawCircle(x,y,ddp*((penWidth/2)*getResources().getDisplayMetrics().density),p);
        c.save();
    }

    @Override
    protected void onDraw(Canvas c) {
        super.onDraw(c);
        if (cacheBitmap != null)
            c.drawBitmap(cacheBitmap,0,0,null);
    }
}
