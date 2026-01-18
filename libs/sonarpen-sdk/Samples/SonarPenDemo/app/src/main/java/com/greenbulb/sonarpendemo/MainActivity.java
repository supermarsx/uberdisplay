package com.greenbulb.sonarpendemo;

import android.Manifest;
import android.app.Dialog;
import android.content.DialogInterface;
import android.content.SharedPreferences;
import android.content.pm.PackageManager;
import android.hardware.usb.UsbDevice;
import android.hardware.usb.UsbManager;
import android.os.Build;
import android.os.Bundle;
import android.os.Environment;
import android.util.Log;
import android.view.KeyEvent;
import android.view.View;
import android.widget.Button;
import android.widget.TextView;
import android.widget.Toast;

import androidx.appcompat.app.AlertDialog;
import androidx.appcompat.app.AppCompatActivity;
import androidx.core.app.ActivityCompat;

import com.greenbulb.sonarpen.SonarPenCallBack;
import com.greenbulb.sonarpen.SonarPenReadings;
import com.greenbulb.sonarpen.SonarPenUtilities;

import java.io.File;
import java.io.FileOutputStream;
import java.text.SimpleDateFormat;
import java.util.Collection;
import java.util.Date;
import java.util.HashMap;

public class MainActivity extends AppCompatActivity implements SonarPenCallBack, MainActivityCallback {
    private AppCompatActivity thisActivity;
    private SonarPenUtilities thisSonarPen;
    private boolean bStartToUseSonarPen=false;
    private boolean bWriteLog=true;
    private boolean bByPassDetection=true;

    private boolean bThreadRunning=false;
    private int penSize=0;

    private String lastRead="";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        thisActivity = this;
        SharedPreferences sh = getSharedPreferences(getPackageName(),MODE_PRIVATE);
        SharedPreferences.Editor ed = sh.edit();
        ed.remove("_sp_readings");
        ed.commit();

        thisSonarPen = new SonarPenUtilities(thisActivity);     // get SonarPenUtilities
        thisSonarPen.setSkipTouchDownEvent();

        thisSonarPen.addSonarPenToView(findViewById(R.id.draw_view));           // add draw_view as SonarPen Support view.

        thisSonarPen.addSonarPenCallback(this);                                 // add current object as SonarPenCallBack.

        ((DrawView) findViewById(R.id.draw_view)).setDelegate(this);

        ((Button) findViewById(R.id.popup_menu)).setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                boolean bPaused = thisSonarPen.getIsAudioPaused();
                boolean bSkipDown = thisSonarPen.isSkipTouchDownEvent();
                boolean bSkipFinger = thisSonarPen.getSkipFingerTouchEvent();
                boolean bLogCatOn = thisSonarPen.getDebugLogStatus();
                int currentFormula = thisSonarPen.getPressureTuneFormula();
                AlertDialog.Builder builder = new AlertDialog.Builder(thisActivity);
                builder.setTitle("Select an action")
                        .setItems(new String[]{(bStartToUseSonarPen?"Stop Sonar Pen":"Start Sonar Pen"),"Clear Screen","Calibrate Sonar Pen",(thisSonarPen.getUseTouchSize()?"Not use Touch size":"use Touch size"),((bByPassDetection)?"By Pass Detection":"No By-pass Detection"),((bPaused)?"Resume Audio":"Pause Audio"),((bSkipDown)?"Touch Down":"Skip Touch Down"),"Use Low Feq",((currentFormula==0)?"curve 1 => 2":"curve 2 => 1"),((penSize==0)?"Big > Calibrate Pen":"Calibrate > Big Pen"),"USB devices",((bSkipFinger)?"to be All Touch Events":"to be SonarPen Event Only"),((bLogCatOn)?"to turn off LogCat":"to turn on LogCat")}, new DialogInterface.OnClickListener() {
                            public void onClick(DialogInterface dialog, int which) {
                                switch (which) {
                                    case 0:         // Start/Stop SonarPen
                                        if (bStartToUseSonarPen) {
                                            StopSonarPen();
                                            bStartToUseSonarPen=false;
                                        } else {
                                            StartSonarPen();
                                            bStartToUseSonarPen=true;
                                        }
                                        if (bWriteLog) writeLog((bStartToUseSonarPen?"Start SonarPen Library":"Stop SonarPen Libraray"));
                                        break;
                                    case 1:         // Select Clear Screen
                                        ((DrawView) findViewById(R.id.draw_view)).clearScreen();
                                        if (bWriteLog) writeLog("Clear Screen");
                                        break;
                                    case 2:         // Select Calibrate Sonar Pen
                                        thisSonarPen.startCalibrateScreen();
                                        break;
                                    case 3:
                                        boolean b = ! thisSonarPen.getUseTouchSize();
                                        thisSonarPen.setUseTouchSize(b);
                                        break;
                                    case 4:
                                        bByPassDetection = ! bByPassDetection;
                                        break;
                                    case 5:
                                        if (thisSonarPen.getIsAudioPaused()) {
                                            thisSonarPen.audioResume();
                                            Toast.makeText(thisActivity,"Audio Has been resume.",Toast.LENGTH_LONG).show();
                                        } else {
                                            thisSonarPen.audioPause();
                                            Toast.makeText(thisActivity,"Audio Has been Paused.",Toast.LENGTH_LONG).show();
                                        }
                                        break;
                                    case 6:
                                        if (bSkipDown) {
                                            thisSonarPen.clearSkipTouchDownEvent();
                                            Toast.makeText(thisActivity, "Normal Touch Down Event",Toast.LENGTH_LONG).show();
                                        } else {
                                            thisSonarPen.setSkipTouchDownEvent();
                                            Toast.makeText(thisActivity, "Skip Touch Down Event",Toast.LENGTH_LONG).show();
                                        }
                                        break;
                                    case 7:
                                        thisSonarPen.sonarpen_msfunc(0x4C46);
                                        Toast.makeText(thisActivity, "force using low Frequency",Toast.LENGTH_LONG).show();
                                        break;
                                    case 8:
                                        if (currentFormula == 0) {
                                            thisSonarPen.setPressureTuneFormula(1);
                                            Toast.makeText(thisActivity, "Change to Curve 2",Toast.LENGTH_LONG).show();
                                        }
                                        else {
                                            thisSonarPen.setPressureTuneFormula(0);
                                            Toast.makeText(thisActivity, "Change to Curve 1",Toast.LENGTH_LONG).show();
                                        }
                                        break;
                                    case 9:
                                        if (penSize == 0) {
                                            ((DrawView) findViewById(R.id.draw_view)).setPenWidth(30);
                                            penSize = 1;
                                            Toast.makeText(thisActivity, "Change Drawing Width as Calibrate App",Toast.LENGTH_LONG).show();
                                        } else {
                                            ((DrawView) findViewById(R.id.draw_view)).setPenWidth(100);
                                            penSize = 0;
                                            Toast.makeText(thisActivity, "Change Drawing Width as Original",Toast.LENGTH_LONG).show();
                                        }
                                        break;
                                    case 10: {
                                        UsbManager m = (UsbManager) thisActivity.getApplicationContext().getSystemService(USB_SERVICE);
                                        HashMap<String, UsbDevice> usbDevices = m.getDeviceList();
                                        Collection<UsbDevice> ite = usbDevices.values();
                                        UsbDevice[] usbs = ite.toArray(new UsbDevice[]{});
                                        String s = "";
                                        Log.v("HERE","usbs="+usbs.length);
                                        for (UsbDevice usb : usbs) {
                                            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.LOLLIPOP) {
                                                s += "\"" + usb.getProductName().toString() + "\"";
                                                if (usb.getProductName().toString().contains("SP1")) {
                                                    s += " is SP1";
                                                }
                                                s += "\n";
                                            } else {
                                                String usbtostr = usb.toString();
                                                int idx = usbtostr.indexOf("mProductName=");
                                                if (idx != -1) {
                                                    idx += 13;
                                                    String pname = usbtostr.substring(idx);
                                                    idx = pname.indexOf(',');
                                                    if (idx != -1) {
                                                        pname = pname.substring(0, idx);
                                                    }
                                                    s += "\"" + pname + "\"";
                                                    if (pname.contains("SP1")) {
                                                        s += " is SP1";
                                                    }
                                                    s += "\n";
                                                }
                                            }
                                        }
                                        ((TextView) findViewById(R.id.usb_text)).setText(s);
                                        }
                                        break;
                                    case 11:
                                        thisSonarPen.setSkipFingerTouchEvent(! bSkipFinger);
                                        break;
                                    case 12:
                                        thisSonarPen.setDebugLogOnOff(! bLogCatOn);
                                        break;
                                }
                            }
                        });
                Dialog popupDialog = builder.create();

                popupDialog.show();
            }
        });

//        Log.v("SONAR_PEN","Version: "+thisSonarPen.VERSION_NO);  // showing Version no. in LogCat
    }

    @Override
    public void onSonarPenButtonPressed() {
        //  if SonarPen pressed, clear drawing screen
        ((DrawView) findViewById(R.id.draw_view)).clearScreen();
        if (bWriteLog) writeLog("SonarPen Button Touch");
    }

    private void writeLog(String msg) {
        Log.v("HERE",msg);
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.P) {
            File extDir = Environment.getExternalStorageDirectory();
            if (extDir == null) {
                return;
            }
            String file_path = extDir.getAbsolutePath() + "/SonarPen";
            File dir = new File(file_path);
            if (!dir.exists()) try {
                dir.mkdirs();
            } catch (Exception e) {
                e.printStackTrace();
            }
            File f = new File(dir, "Log.txt");
            try {
                FileOutputStream fout = new FileOutputStream(f, true);
                SimpleDateFormat sdf = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss");
                String outstr = sdf.format(new Date()) + " " + msg + "\r\n";
                fout.write(outstr.getBytes("UTF-8"));
                fout.close();
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }

    @Override
    public void touchevent_call() {
        /*
        SonarPenReadings readings = new SonarPenReadings();
        if (thisSonarPen != null) {
            thisSonarPen.getCurrentReadingValue(readings);
            runOnUiThread(new showReadings(readings));
        }
         */
    }

    private class showReadings implements Runnable {
        private SonarPenReadings readings;

        public showReadings(SonarPenReadings r) {
            readings = r;
        }

        public void run() {
            String msg = "last: "+lastRead+", curr:"+readings.currentValue+", pressure:"+readings.pressure+", min:"+readings.minValue+", max:"+readings.maxValue+", calibrate min:"+readings.manualMin+", calibrate max:"+readings.manualMax+", touch:"+readings.touchMinValue+", read status="+readings.audioReadStatus+", low freq:"+(readings.bLowFreq?"Y":"N")+" ("+readings.currentFeq+"), vol:"+readings.currentSoundVol+" "+readings.extraInfo;
            ((TextView) findViewById(R.id.calibrate_text)).setText(msg);
        }
    }

    private class setStatusText implements Runnable {
        // show message on status text label while runOnUiThread
        private String msg="";
        public setStatusText(String s) {
            msg = s;
            writeLog(s);
        }

        public void run() {
            ((TextView) findViewById(R.id.status_text)).setText(msg);
        }
    }

    @Override
    public void onSonarPenStatusChange(int state) {

        // status change callback from SonarPen library.
        String msg="";
        switch (state) {
            case SonarPenCallBack.INIT_STAGE:
                msg = "Library Initization";
                break;
            case SonarPenCallBack.CALIBRATE_SONAR_PEN:
                msg = "Sonar Pen Calibrated";
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
                if (thisSonarPen.isSonarPenOnScreen()) msg = "SonarPen touch on screen";
                break;
            case SonarPenCallBack.WAITING_HEADSET:
                msg = "Waiting Headset plug online";
                break;
            case SonarPenCallBack.SONAR_PEN_NOT_PLUGED: {
//                SharedPreferences sh = getSharedPreferences(getPackageName(),MODE_PRIVATE);
//                String s = sh.getString("_sp_readings","");
//                if (s.length() > 0) {
//                    msg = s + "\n"+Build.BRAND+"\nPlugged device is not SonarPen";
//                } else {
                    msg = "Plugged device is not SonarPen";
                }
//                }
                break;
            case SonarPenCallBack.AUDIO_CHANNEL_FAIL:
                msg = "Audio Chnnel Fail to Open";
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

        if (thisSonarPen.isUsingManualCalibrate()) msg += " (* manual calibrated *)";
        if (thisSonarPen.getUseTouchSize()) msg += " (* touchsize *)";

        runOnUiThread(new setStatusText(msg));
    }

    private class readingThread implements Runnable {
        public void run() {
            SonarPenReadings readings = new SonarPenReadings();
            while ((bThreadRunning) && (thisSonarPen != null)) {
                thisSonarPen.getCurrentReadingValue(readings);
                SimpleDateFormat sdf = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss");
                lastRead = sdf.format(new Date());
                runOnUiThread(new showReadings(readings));
                try {
                    Thread.sleep(500);
                } catch (Exception e) {}
            }
        }
    }
    private void StopSonarPen() {
        bThreadRunning=false;
        if (thisSonarPen != null)
            thisSonarPen.stop();
    }

    private void StartSonarPen() {
//        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.LOLLIPOP) {
//            // try to retrieve uses-permission.
//            if ((ActivityCompat.checkSelfPermission(thisActivity, Manifest.permission.RECORD_AUDIO) != PackageManager.PERMISSION_GRANTED) ||
//                    (ActivityCompat.checkSelfPermission(thisActivity, Manifest.permission.READ_EXTERNAL_STORAGE) != PackageManager.PERMISSION_GRANTED) ||
//                    (ActivityCompat.checkSelfPermission(thisActivity, Manifest.permission.WRITE_EXTERNAL_STORAGE) != PackageManager.PERMISSION_GRANTED)) {
//
//                ActivityCompat.requestPermissions(thisActivity, new String[]{Manifest.permission.RECORD_AUDIO, Manifest.permission.WRITE_EXTERNAL_STORAGE,Manifest.permission.READ_EXTERNAL_STORAGE},
//                        12345);
//            }
//
//        }
        // Start SonarPen Library.
        if (bByPassDetection) thisSonarPen.startWithByPass();
        else {
            thisSonarPen.setByPassDetectionAsDefault(false);
            thisSonarPen.start();
        }
        bThreadRunning=true;
        new Thread(new readingThread()).start();
    }

    @Override
    public void onResume() {
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
        if (bStartToUseSonarPen) {
            thisSonarPen.reloadOnResume();
            StartSonarPen();
        }
        if (bWriteLog) writeLog("Activity Resume");
        super.onResume();
        // if no Permission error
//        if (thisSonarPen.getState() == thisSonarPen.ERROR_NO_PERMISSION) {
//            Log.v("SONAR_PEN","No Audio Record Access Permission");
//        }
    }

    @Override
    public void onPause() {
        // Stop SOnarPen Library, restore Media Volumne.
        StopSonarPen();
        if (bWriteLog) writeLog("Activity Pause");
        super.onPause();
    }

    @Override
    public boolean onKeyUp(int keyCode, KeyEvent event) {
        // thisSonarPen.isSonicPenButton(event), call this let SonarPen Library handle SonarPen Button, if true, the event is SonarPen Button.
        if (thisSonarPen != null)
            if (thisSonarPen.isSonicPenButton(event)) return true;
        return super.onKeyUp(keyCode, event);
    }

    @Override
    public boolean onKeyDown(int keyCode, KeyEvent event) {
        // thisSonarPen.isSonicPenButton(event), call this let SonarPen Library handle SonarPen Button, if true, the event is SonarPen Button.
        if (thisSonarPen != null)
            if (thisSonarPen.isSonicPenButton(event)) return true;
        return super.onKeyDown(keyCode, event);
    }

}
