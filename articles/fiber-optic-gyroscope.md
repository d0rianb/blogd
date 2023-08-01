---
author: Dorian Beauchesne
date: 01/08/2023
title: Fiber Optic Gyroscope
language: en
---

# The principle of Fiber Optic Gyroscope
> Gyroscope are part of our today life, without us event knowing it.

## Whats is a gyroscope and and IMU
An **IMU** (Inertial Mesurement Unit) is *"an electronic device that measures and reports a body's specific force, angular rate using a combination of accelerometers, gyroscopes"*[^1]. It helps devices to know their spatial orientation and movements.  
They are part of smartphones, aircrafts, rockets and satellites. In fact, this is an IMU defect that cause the crash of the **Ariane 5** during its first flight.
An IMU basically consists in 2 sensor types : accelerometers and gyroscopes. The accelerometers are used for mesuring translations and the gyroscopes for the rotations. 
Every IMU is composed with 3 acceloremeters and 3 gyroscopes : one for each axis (`X`, `Y` and `Z`)

![Degrees of Freedom of an IMU](https://miro.medium.com/v2/resize:fit:720/format:webp/0*bS8im7IxVmW4DK9o.jpg)  
*Diagram of the elementary motions in 3D space*

### Accelerometers
An accelerometer is a simple sensor that uses the Netwon's second law : $ \mathrm{ \vec F = \sum{m \vec a}} $.   
$m$ (kg) beeing the mass of the solid, $\vec F$ (N) beeing the resultant of all forces applied to this solid and $\vec a$ (m/s²) beeing the acceleration of the solid.  

![Basic principle of an accelerometer](https://www.vectornav.com/inertialprimer/support-library/imu_accel.jpg)
*Principle of a mechanical spring-mass accelerometer*

Acceleration cannot be directly measured but force can be and since they are proportional, the conversion is easily done.  
There is multiple tecnhologies of accelerometers, some use the piezoelectric effects, others the Hall effects, but the famous ones are the **MEMS** *(Microelectromechanical systems)*. **MEMS** consist in a integrated circuit and microsensors that use the capacitive electro-mechanical properties to detect forces. They have the advantage of beeing miniaturized so they fit perfectly in smartphones or watches.

![Diagram of a MEMS](https://www.siliconsensing.com/media/1450/gemini-gif.gif?width=500&height=400)  
*Illustration of a working MEMS*

### Gyroscopes
The gyroscope is far older than the accelerometer and illustrate the principle of conservation of angular momentum : $ \mathrm{{\vec L}_{\mathrm O } = \sum{\overrightarrow {\mathrm OM }}\wedge {\vec p}} $.   
This means that when a solid is rotating, its will is to remain in this state. This is why we can stand on a moving bike. 
> "The main properties that an object can experience in any gyroscopic motion are rigidity in space and precession"[^2]. 

![Gyroscope GIF](https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Gyroscope_operation.gif/220px-Gyroscope_operation.gif)  
*A gyroscope in operation, showing the freedom of rotation in all three axes. The rotor will maintain its spin axis direction regardless of the orientation of the outer frame.*[^2]

Gyroscopes are used for stabilization purpose (ex: Steadicam), Heading indicator in aircrafts, calculation the Earth rotation (Gyrocompass), so basically every navigation & orientation fields. We all have gyroscope in our phones to perform screen rotation or to allow developper to developp usefull apps. 
![iBeer](https://img.phonandroid.com/2022/01/ibeer-app-iPhone.jpg)  
*The best application of the iPhone 3G*

Mechanical gyroscopes (with a spinning wheel) were the first discovered. They permit to measure a rotation relative to the fixed axis of the wheel. But they also have other usages. For instance, the telescope **Hubble** uses internal gyroscopes to orient itself precisely in the vacuum of space.  

![Hubble gyroscopes](https://i.insider.com/5bbbbad066fb3f10535e16b3?width=750&format=jpeg&auto=webp)  
***Hubble**'s pointing control system* 

But mechanical technologies suffer from vibrations and frictions which lead to innaccuracy and wear. So other types of gyroscope were developped, based on others physics principles : 
 - **MEMS gyroscope** : idea of the *Foucault pendulum* using a vibrating element
 - **VSG** (vibrating structure gyroscope) : uses a resonator made of different metallic alloys
 - **Ring laser gyroscope** : relies on the *Sagnac effect* to measure rotation by measuring the shifting interference pattern of a beam split into two separate beams which travel around the ring in opposite directions
 - **London moment** : relies on the quantum-mechanical phenomenon, whereby a spinning superconductor generates a magnetic field whose axis lines up exactly with the spin axis of the gyroscopic rotor
 - **FOG (Fiber optic gyroscope)**: The one we're interested in today

#### FOG
 The **FOG**, as the **Ring Laser Gyroscope** uses the *Sagnac effect* to detect changes in orientation. The *Sagnac effect* is an interferometry phenomenon that appears between two light beams turning in the opposite direction : $ \mathrm{ \Delta \Phi_{R} = {2\pi DL \over \lambda c}}  $ 

 ![Diagram of the Sagnac effect](https://www.researchgate.net/publication/333975780/figure/fig1/AS:773294076723200@1561379083122/A-fiber-optic-gyroscope-simplified-view-with-a-single-half-loop.png)  
 *A fiber optic gyroscope (simplified view with a single half-loop)*

When the gyroscope turns, one beams will have more distance to travel than the other to perform an entire revolution. This path difference will cause interferences that can be measured by a **photodiode**.

![Path difference diagram](https://upload.wikimedia.org/wikipedia/commons/thumb/e/e0/Sagnac_shift.svg/220px-Sagnac_shift.svg.png)  
*Light traveling opposite directions go different distances before reaching the moving source*

The **FOG** provides extremely precise rotational rate information, in part because of its lack of cross-axis sensitivity to vibration, acceleration, and shock. But on the other hand, it require very precise calibration and a complex manufacturing process. 



[^1] https://en.wikipedia.org/wiki/Inertial_measurement_unit  
[^2] https://en.wikipedia.org/wiki/Gyroscope  

