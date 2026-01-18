package com.greenbulb.sonarpendemo2;

import android.Manifest;
import android.app.Dialog;
import android.content.DialogInterface;
import android.content.pm.PackageManager;
import android.os.Build;
import android.os.Bundle;
import android.util.Log;
import android.view.KeyEvent;
import android.view.MotionEvent;
import androidx.appcompat.app.AppCompatActivity;
import androidx.core.app.ActivityCompat;

import android.view.View;
import android.widget.Button;
import android.widget.TextView;
import android.widget.Toast;

import com.greenbulb.sonarpen.SonarPenActivity;
import com.greenbulb.sonarpen.SonarPenCallBack;
import com.greenbulb.sonarpen.SonarPenUtilities;

import org.json.JSONObject;


// using SonarPenActivity sample.
// all touch event will be through SonarPen Library, therefore onClick event will be effected.
public class MainActivity extends AppCompatActivity implements SonarPenCallBack,TouchCallBack {
    private AppCompatActivity thisActivity;
    private SonarPenUtilities thisSonarPen;
    private boolean byPassDetection=false;      // set to true by pass Sonar Pen Detection.

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        thisSonarPen = new SonarPenUtilities(this);     // get SonarPenUtilities
        thisSonarPen.setByPassDetectionAsDefault(byPassDetection);        // override default by Pass Sonar Pen Detection
        thisActivity = this;

        thisSonarPen.addSonarPenToView(findViewById(R.id.touch_view));           // add draw_view as SonarPen Support view.

        ((TouchView) findViewById(R.id.touch_view)).assignInterface(this);

        thisSonarPen.addSonarPenCallback(this);                                 // add current object as SonarPenCallBack.
    }

    @Override
    public void onResume() {
        // On resume, try to retrieve uses-permission.
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.LOLLIPOP) {
            // try to retrieve uses-permission.
            if (ActivityCompat.checkSelfPermission(this, Manifest.permission.RECORD_AUDIO) != PackageManager.PERMISSION_GRANTED)  {
                ActivityCompat.requestPermissions(this, new String[]{Manifest.permission.RECORD_AUDIO},
                        12345);
            }
            if (Build.VERSION.SDK_INT < Build.VERSION_CODES.P) {
                if ((ActivityCompat.checkSelfPermission(this, Manifest.permission.READ_EXTERNAL_STORAGE) != PackageManager.PERMISSION_GRANTED) ||
                        (ActivityCompat.checkSelfPermission(this, Manifest.permission.WRITE_EXTERNAL_STORAGE) != PackageManager.PERMISSION_GRANTED)) {
                    ActivityCompat.requestPermissions(this, new String[]{Manifest.permission.WRITE_EXTERNAL_STORAGE, Manifest.permission.READ_EXTERNAL_STORAGE},
                            12346);
                }
            }
            if (ActivityCompat.checkSelfPermission(this, "sonarpen.calibrate.READ_PERMISSION") != PackageManager.PERMISSION_GRANTED) {
                if (ActivityCompat.shouldShowRequestPermissionRationale(this,"sonarpen.calibrate.READ_PERMISSION")) {
                    ActivityCompat.requestPermissions(this, new String[]{"sonarpen.calibrate.READ_PERMISSION"},12347);
                }
            }
        }
        thisSonarPen.reloadOnResume();
        if (byPassDetection) thisSonarPen.startWithByPass();
        else thisSonarPen.start();
        if (thisSonarPen.getState() == thisSonarPen.ERROR_NO_PERMISSION) {
            Log.v("SONAR_PEN","No Audio Record Access Permission");
        }
        super.onResume();
    }

    @Override
    public void onPause() {
        super.onPause();
        thisSonarPen.stop();
    }

    @Override
    public void onSonarPenButtonPressed() {
        // SonarPen Button Pressed
        ((TextView) findViewById(R.id.message_text)).append("\n Button Pressed");
    }

    private class setStatusText implements Runnable {
        private String msg="";
        public setStatusText(String s) {
            msg = s;
        }

        public void run() {
            ((TextView) findViewById(R.id.status_text)).setText(msg);
        }
    }

    @Override
    public void onSonarPenStatusChange(int state) {
        // SonarPen Status Changed.
        String msg="";
        switch (state) {
            case SonarPenCallBack.INIT_STAGE:
                msg = "Library Initization";
                break;
            case SonarPenCallBack.CALIBRATE_SONAR_PEN:
                msg = "Start Calibrate Sonar Pen";
                break;
            case SonarPenCallBack.CALIBRATE_STAGE:
                msg = "Start Calibrate Preparation";
                break;
            case SonarPenCallBack.CLOSED_STAGE:
                msg = "Stop Library";
                break;
            case SonarPenCallBack.INIT_FAIL:
                msg = "Library Initization fail";
                break;
            case SonarPenCallBack.SONAR_PEN_NOT_PRESS:
                msg = "SonarPen point not press";
                break;
            case SonarPenCallBack.SONAR_PEN_PRESSED:
                msg = "SonarPen point pressed";
                if (thisSonarPen.isSonarPenOnScreen()) msg = "SonarPen on Screen";
                break;
            case SonarPenCallBack.WAITING_HEADSET:
                msg = "Waiting Headset plug online";
                break;
            case SonarPenCallBack.SONAR_PEN_NOT_PLUGED:
                msg = "Plugged Device is not Sonar Pen";
                break;
            case SonarPenCallBack.AUDIO_CHANNEL_FAIL:
                msg = "Audio Channel Fail to Open";
                break;
            case SonarPenCallBack.SONAR_PEN_NOT_DETECTED_BY_PASSED:
                msg = "By Passed detect logic fail";
                break;
            case SonarPenCallBack.AUDIO_VOL_CANT_CHANGE:
                msg = "Audio Volume Can\'t Change";
                break;
            default:
                msg = "Unknown :"+state;
                break;
        }

        if (thisSonarPen.isUsingManualCalibrate()) msg += " (* using manual calibrate *)";

        runOnUiThread(new setStatusText(msg));
    }

    private class ShowMessage implements Runnable {
        private String msg="";

        public ShowMessage(String text) {
            msg = text;
        }

        public void run() {
            ((TextView) findViewById(R.id.message_text)).setText(msg);
        }
    }

    private class DisplayReading implements Runnable {
        private JSONObject obj;

        public DisplayReading(JSONObject o) {
            obj = o;
        }

        public void run() {
            String s = "";
            try {
                s += "current:"+obj.getDouble("currentValue")+", max:"+obj.getDouble("maxValue")+", in file:"+obj.getDouble("maxManualValue");
            } catch (Exception e) {};

            ((TextView) findViewById(R.id.reading_text)).setText(s);
        }
    }


    private class buildUpMessage implements Runnable {
        private MotionEvent evt;

        public buildUpMessage(MotionEvent event) {
            evt =event;
        }

        public void run() {
            String text = "";
            if (evt.getActionMasked() == MotionEvent.ACTION_DOWN) {
                text = "TOUCH DOWN\n";
            } else if (evt.getActionMasked() == MotionEvent.ACTION_UP) {
                text = "TOUCH UP\n";
            } else if (evt.getActionMasked() == MotionEvent.ACTION_MOVE) {
                text = "TOUCH MOVE\n";
            } else if (evt.getActionMasked() == MotionEvent.ACTION_POINTER_DOWN) {
                text = "POINTER DOWN\n";
            } else if (evt.getActionMasked() == MotionEvent.ACTION_POINTER_UP) {
                text = "POINTER UP\n";
            } else {
                text = "ACTION ("+evt.getActionMasked()+")\n";
            }
            MotionEvent.PointerCoords corrds = new MotionEvent.PointerCoords();
            MotionEvent.PointerProperties props = new MotionEvent.PointerProperties();

            text += "AUDIO Paused:"+(thisSonarPen.getIsAudioPaused()?"YES":"NO")+"\n";
            for (int i=0;i<evt.getPointerCount();i++) {
                try {
                    evt.getPointerProperties(i, props);
                    evt.getPointerCoords(i, corrds);
                    if (props.toolType == MotionEvent.TOOL_TYPE_STYLUS) {
                        new Thread(new Runnable() {
                            public void run() {
                                JSONObject obj = thisSonarPen.getCurrReading();

                                runOnUiThread(new DisplayReading(obj));
                            }
                        }).start();
                    }
                    text += "POINTER (" + i + ") (" + corrds.x + "," + corrds.y + ")\n\tID=" + evt.getPointerId(i) + "\n\tDeviceID=" + evt.getDeviceId() + "\n\tpressure " + String.format("%.6f", corrds.pressure) + "\n\ttype=" + ((props.toolType == MotionEvent.TOOL_TYPE_STYLUS) ? "Stylus" : "Finger") + "\n\tsize=" + String.format("%.6f", corrds.size) + "\n";
                } catch (Exception e) {};
            }

            runOnUiThread(new ShowMessage(text));
        }
    }

    @Override
    public void receiveMotionEvent(MotionEvent event) {
        new Thread(new buildUpMessage(event)).start();
    }

    @Override
    public boolean dispatchTouchEvent(MotionEvent motionEvent) {
        return super.dispatchTouchEvent(thisSonarPen.translateTouchEvent(motionEvent));
    }

    @Override
    public boolean onKeyUp(int keyCode, KeyEvent event) {
        if (thisSonarPen.isSonicPenButton(event)) return true;
        return super.onKeyUp(keyCode, event);
    }

    @Override
    public boolean onKeyDown(int keyCode, KeyEvent event) {
        if (thisSonarPen.isSonicPenButton(event)) return true;
        return super.onKeyDown(keyCode, event);
    }

}
