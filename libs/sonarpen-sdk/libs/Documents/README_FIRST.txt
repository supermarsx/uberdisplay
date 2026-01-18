/******************************************************************************************/
/***                                                                                    ***/
/***    Sonar Pen Android Library Setting                                               ***/
/***    Version: 3.11 (2024042101)                                                      ***/
/***    ---------------------------------------------------------------------------     ***/
/***    Copyrights GreenBlub                                                            ***/
/***                                                                                    ***/
/******************************************************************************************/
/***    Manual Calibrate Readings                                                       ***/
/***    - for some Device that the SonarPen pressure not handle in good conditions      ***/
/***                                                                                    ***/
/******************************************************************************************/

-- Android Studio Project Setup
    1. Copy the sonarpen.aar into "libs" directory of your project.
    2. add the followings repository section into your build.gradle file :-
        repositories {
            flatDir { dirs 'libs' }
        }
    3. add the following line into dependency section of your build.gradle file :-
        implementation fileTree(dir: "libs", include: ["*.jar","*.aar"])            // Android Studio Hedgehog or latest
        -- or -- 
        implementation (name:'sonarpen', ext:'aar')                                 // Android Studio version 3 or above
        -- or --
        compile  (name:'sonarpen', ext:'aar')                                       // Android Studio below version 3
        
    4.  Clean your project.

    Note, you are require to have android.permission.RECORD_AUDIO permission in your application for reading SonarPen pressure.
          you are require to have sonarpen.calibrate.READ_PERMISSION permission with queries to sonarpen.calibrate in your application for retrieve Calibrate data.
          you are require to have android.permission.WRITE_EXTERNAL_STORAGE permission in your application if your application will provide Manual Calibrate interface and save Manual Calibrate Readings.
          you are require to have android.permission.READ_EXTERNAL_STORAGE permission in  your application if your application will read in the Manual Calibrate Readings.
          in AndroidManifest.xml

          <uses-permission android:name="android.permission.RECORD_AUDIO"/>
          <uses-permission android:name="android.permission.WRITE_EXTERNAL_STORAGE"/>
          <uses-permission android:name="android.permission.READ_EXTERNAL_STORAGE"/>
          <uses-permission android:name="sonarpen.calibrate.READ_PERMISSION"/>
          <queries>
               <provider android:authorities="sonarpen.calibrate" />
          </queries>


-- Include SonarPen into your Activity.
    there is two way.
    1. Replace AppCompatActivity with SonarPenActivity.
    2. Add SonarPenUtilities manually.
        a.  create a new SonarPenUtilities object.
        b.  add SonarPenUtilities.start() in onResume()
        c.  add SonarPenUtilities.reloadOnResume() in onResume() to make sure calibrate data is retrieved.
        d.  add SonarPenUtilities.stop() in onPause()
        e.  add "if (SonarPenUtilities.isSonicPenButton(event)) return true;" in onKeyDown(KeyEvent event) and onKeyUp(KeyEvent event)
        f.  use addSonarPenToView(view), to mark the view that will have Sonar Pen input.

-- Include SonarPen support into your View.
    use SonarPenActivity.addSonarPenToView(your_view);

    SonarPenActivity will translate TouchEvent and attach pressure and Touch ToolType as Stylus, where ths touch point is Sonar Pen point.

-- Handle SonarPen Button.
    use SonarPenCallBack interface.
    when SonarPen Button is pressed, the function onSonarPenButtonPressed() will fire.

    note, use SonarPenUtilities.addSonarPenCallback(callbackhandler) to let the library to attach with your interface.

-- Manual Calibrate Readings, file is store in "SonarPen" folder of deivce's internal Storage, only on Android 10 or below devices.


