-- Seed Data for 5BX Charts and Instructions

-- 72 Levels across 6 Charts
INSERT INTO exercise_charts (chart, level, ex1, ex2, ex3, ex4, ex5, ex5_run, ex5_walk) VALUES
  (1, 1, 2, 3, 4, 2, 100, 480, 1260),
  (1, 2, 3, 4, 5, 3, 145, 450, 1260),
  (1, 3, 4, 5, 6, 3, 175, 420, 1200),
  (1, 4, 6, 7, 8, 4, 205, 390, 1140),
  (1, 5, 7, 8, 10, 5, 235, 390, 1140),
  (1, 6, 8, 9, 12, 6, 260, 390, 1140),
  (1, 7, 10, 11, 13, 7, 280, 360, 1080),
  (1, 8, 12, 12, 14, 8, 305, 360, 1080),
  (1, 9, 14, 13, 15, 9, 320, 360, 1080),
  (1, 10, 16, 15, 16, 11, 335, 330, 1020),
  (1, 11, 18, 17, 17, 12, 375, 330, 1020),
  (1, 12, 20, 18, 18, 13, 400, 330, 1020),
  (2, 1, 14, 10, 13, 9, 335, 630, 2100),
  (2, 2, 15, 11, 14, 10, 360, 630, 2100),
  (2, 3, 16, 12, 15, 11, 380, 630, 2100),
  (2, 4, 18, 13, 17, 12, 395, 600, 2040),
  (2, 5, 19, 14, 19, 13, 410, 600, 2040),
  (2, 6, 20, 15, 21, 14, 425, 600, 2040),
  (2, 7, 22, 16, 23, 15, 440, 570, 1980),
  (2, 8, 24, 17, 25, 16, 445, 570, 1980),
  (2, 9, 26, 18, 27, 17, 455, 570, 1980),
  (2, 10, 28, 20, 29, 18, 470, 540, 1920),
  (2, 11, 29, 21, 31, 19, 485, 540, 1860),
  (2, 12, 30, 23, 33, 20, 500, 540, 1800),
  (3, 1, 24, 20, 29, 15, 400, 525, 1740),
  (3, 2, 24, 21, 30, 15, 415, 525, 1680),
  (3, 3, 24, 22, 31, 15, 430, 525, 1680),
  (3, 4, 26, 23, 33, 16, 450, 510, 1620),
  (3, 5, 26, 24, 34, 17, 465, 510, 1620),
  (3, 6, 26, 25, 35, 17, 480, 510, 1620),
  (3, 7, 28, 26, 37, 18, 490, 495, 1560),
  (3, 8, 28, 27, 39, 19, 500, 495, 1560),
  (3, 9, 28, 28, 41, 20, 510, 495, 1560),
  (3, 10, 30, 30, 43, 21, 525, 480, 1500),
  (3, 11, 30, 31, 45, 22, 540, 480, 1500),
  (3, 12, 30, 32, 47, 24, 550, 480, 1500),
  (4, 1, 24, 18, 40, 17, 300, 465, 1380),
  (4, 2, 24, 18, 40, 19, 315, 465, 1380),
  (4, 3, 24, 18, 41, 21, 325, 465, 1380),
  (4, 4, 26, 19, 43, 24, 335, 450, 1260),
  (4, 5, 26, 19, 43, 26, 345, 450, 1260),
  (4, 6, 26, 19, 44, 28, 355, 450, 1260),
  (4, 7, 28, 21, 46, 30, 365, 435, 1200),
  (4, 8, 28, 21, 46, 32, 375, 435, 1200),
  (4, 9, 28, 21, 47, 34, 380, 435, 1200),
  (4, 10, 30, 22, 49, 37, 390, 420, 1140),
  (4, 11, 30, 22, 49, 40, 395, 420, 1140),
  (4, 12, 30, 22, 50, 42, 400, 420, 1140),
  (5, 1, 24, 26, 39, 30, 375, 420, 0),
  (5, 2, 24, 27, 40, 31, 385, 411, 0),
  (5, 3, 24, 28, 41, 32, 400, 405, 0),
  (5, 4, 26, 30, 42, 34, 410, 399, 0),
  (5, 5, 26, 31, 43, 35, 420, 393, 0),
  (5, 6, 26, 32, 44, 36, 435, 387, 0),
  (5, 7, 28, 34, 45, 38, 445, 381, 0),
  (5, 8, 28, 35, 46, 39, 455, 375, 0),
  (5, 9, 28, 36, 47, 40, 465, 372, 0),
  (5, 10, 30, 38, 48, 42, 475, 369, 0),
  (5, 11, 30, 39, 49, 43, 485, 366, 0),
  (5, 12, 30, 40, 50, 44, 500, 360, 0),
  (6, 1, 24, 35, 29, 26, 450, 360, 0),
  (6, 2, 24, 36, 30, 27, 460, 351, 0),
  (6, 3, 24, 37, 31, 28, 475, 345, 0),
  (6, 4, 26, 39, 32, 30, 485, 339, 0),
  (6, 5, 26, 40, 33, 31, 495, 333, 0),
  (6, 6, 26, 41, 34, 32, 505, 327, 0),
  (6, 7, 28, 43, 35, 34, 515, 321, 0),
  (6, 8, 28, 44, 36, 35, 525, 315, 0),
  (6, 9, 28, 45, 37, 36, 530, 312, 0),
  (6, 10, 30, 47, 38, 38, 555, 309, 0),
  (6, 11, 30, 48, 39, 39, 580, 306, 0),
  (6, 12, 30, 50, 40, 40, 600, 300, 0);

-- Exercise Instructions
INSERT INTO exercise_instructions (chart, exercise, name, instructions, image_path) VALUES
  (1, 1, 'Toe Touch', 'Feet astride, arms upward. Forward bend to floor touching then stretch upward and backward bend.
Do not strain to keep knees straight.', 'c1_ex1.png'),
  (1, 2, 'Knee Prepare', 'Back lying, feet 6" apart, arms at sides.
Sit up just far enough to see your heels.
Keep legs straight, head and shoulders must clear the floor.', 'c1_ex2.png'),
  (1, 3, 'Back Arch (Alternate Leg Raising)', 'Front lying, palms placed under the thighs.
Raise head and one leg, repeat using legs alternately.
Keep leg straight at the knee, thighs must clear the palms.
(Count one each time second leg touches floor.)', 'c1_ex3.png'),
  (1, 4, 'Knee Push-up', 'Front lying, hands under the shoulders, palms flat on the floor.
Straighten arms lifting upper body, keeping the knees on the floor.
Bend arms to lower body.
Keep body straight from the knees, arms must be fully extended.
Chest must touch floor to complete one movement.', 'c1_ex4.png'),
  (1, 5, 'Run & Scissor Jumps', 'Stationary run - (count a step each time left foot touches floor.Lift feet approximately 4 inches off floor).
Every 75 steps do 10 "scissor jumps". Repeat this sequence until required number of steps is completed.

Scissor Jumps:
Stand with right leg and left arm extended forward, and left leg and right arm extended backward.
Jump up - change position of arms and legs before landing.
Repeat (arms shoulder high).', 'c1_ex5.png'),
  (1, 6, 'Run 1/2 Mile (0.8km)', 'Run the specified distance on a level track or treadmill. Maintain a steady pace aimed at meeting the target time.', 'run.png'),
  (1, 7, 'Walk 1 Mile (1.6km)', 'Walk the specified distance briskly. Maintain a steady pace aimed at meeting the target time.', 'walk.png'),
  (2, 1, 'Toe Touch', 'Feet astride, arms upward.
Touch floor and press (bounce) once then stretch upward and backward bend.
Do not strain to keep knees straight.', 'c2_ex1.png'),
  (2, 2, 'Sit-up (Arms at Sides)', 'Back lying, feet 6" apart, arms at sides.
Sit up to vertical position, keep feet on floor even if it is necessary to hook them under a chair.
Allow knees to bend slightly.', 'c2_ex2.png'),
  (2, 3, 'Back Arch (Both Legs Raising)', 'Front lying, palms placed under the thighs.
Raise head, shoulders and both legs.
Keep legs straight, both thighs must clear the palms.', 'c2_ex3.png'),
  (2, 4, 'Push-up', 'Front lying, hands under the shoulders, palms flat on the floor.
Straighten arms to lift body with only palms and toes on the floor.
Back straight.
Chest must touch floor for each completed movement after arms have been fully extended.', 'c2_ex4.png'),
  (2, 5, 'Run & Star Jumps', 'Stationary run - (count a step each time left foot touches floor. Lift feet approximately 4 inches off floor).
Every 75 steps do 10 "Astride jumps". Repeat this sequence until required number of steps is completed.

Astride Jumps: 
Feet together, arms at side. 
Jump and land with feet astride and arms raised sideways to slightly above shoulder height. 
Return with a jump to the starting position for count of one. Keep arms straight.', 'c2_ex5.png'),
  (2, 6, 'Run 1 Mile (1.6km)', 'Run the specified distance on a level track or treadmill. Maintain a steady pace aimed at meeting the target time.', 'run.png'),
  (2, 7, 'Walk 2 Mile (3.2km)', 'Walk the specified distance briskly. Maintain a steady pace aimed at meeting the target time.', 'walk.png'),
  (3, 1, 'Toe Touch (to Sides)', 'Feet astride, arms upward.
Touch floor 6" outside left foot, again between feet and press once then 6" outside right foot, bend backward as far as possible, repeat, reverse direction after half the number of counts.
Do not strain to keep knees straight, return to erect position.', 'c3_ex1.png'),
  (3, 2, 'Sit-up (Hands Behind Head)', 'Back lying, feet 6" apart, arms clasped behind head.
"Sit up" to vertical position, keep feet on floor even if it is necessary to hook them under a chair.', 'c3_ex2.png'),
  (3, 3, 'Back Extension (Hands Behind Back)', 'Front lying, hands interlocked behind the back.
Lift head, shoulders, chest and both legs as high as possible.
Keep legs straight, and raise chest and both thighs completely off floor.', 'c3_ex3.png'),
  (3, 4, 'V Push-up', 'Front lying, hands under the shoulders, palms flat on the floor.
Touch chin to floor in front of hands.
Touch forehead to floor behind hands before returning to up position.
There are three definite movements: chin, forehead, arms straightened.
DO NOT do in one continuous movement.', 'c3_ex4.png'),
  (3, 5, 'Run & Half Knee Bends', 'Stationary run - (count a step each time left foot touches floor. Lift feet approximately 4 inches off floor).
Every 75 steps do 10 "half knee bends". Repeat this sequence until required number of steps is completed.

Half knee bends:
Feet together, hands on hips, knees bent to form an angle of about 110 degrees.
Do not bend knees past a right angle.
Straighten to upright position, raising heel off floor, return to starting position each time.
Keep feet in contact with floor, the back upright and straight at all times.', 'c3_ex5.png'),
  (3, 6, 'Run 1 Mile (1.6km)', 'Run the specified distance on a level track or treadmill. Maintain a steady pace aimed at meeting the target time.', 'run.png'),
  (3, 7, 'Jog 2 Mile (3.2km)', 'Jog the specified distance briskly. Maintain a steady pace aimed at meeting the target time.', 'walk.png'),
  (4, 1, 'Toe Touch (to Sides and Circle)', 'Feet astride, arms upward.
Touch floor outside left foot, between feet, press once then outside right foot.
Circle bend backwards as far as possible, reverse direction after half the number of counts.
Do not strain to keep knees straight.
Keep arms above head and make full circle, bending backward past vertical each time.', 'c4_ex1.png'),
  (4, 2, 'Sit-up (Touch Toes)', 'Back lying, legs straight, feet together, arms straight overhead.
Sit up and touch the toes keeping the arms and legs straight.
Use chair to hook feet under only if necessary.', 'c4_ex2.png'),
  (4, 3, 'Back Extension (Arms Extended Sideways)', 'Front lying, hands and arms stretched sideways.
Lift head, shoulders, arms, chest and both legs as high as possible.
Keep legs straight, raise chest and both thighs completely off floor.', 'c4_ex3.png'),
  (4, 4, 'Wide Push-up', 'Front lying, palms of hands flat on floor, approximately 1 foot from ears directly to side of head.
Straighten arms to lift body.
Chest must touch floor for each completed movement.', 'c4_ex4.png'),
  (4, 5, 'Run & Semi-Squat Jumps', 'Stationary run - (count a step each time left foot touches floor. Lift feet approximately 4 inches off floor).
Every 75 steps do 10 "semi-squat jumps". Repeat this sequence until required number of steps is completed.

Semi-squat jumps:
Drop to a half crouch position with hands on knees and arms straight, keep back as straight as possible, right foot slightly ahead of left.
Jump to upright position with body straight and feet leaving floor,reverse position of feet before landing.
Return to half crouch position and repeat.', 'c4_ex5.png'),
  (4, 6, 'Run 1 Mile (1.6km)', 'Run the specified distance on a level track or treadmill. Maintain a steady pace aimed at meeting the target time.', 'run.png'),
  (4, 7, 'Jog 2 Mile (3.2km)', 'Jog the specified distance briskly. Maintain a steady pace aimed at meeting the target time.', 'walk.png'),
  (5, 1, 'Toe Touch (to Sides and Circle)', 'Feet astride, arms upward, hands clasped, arms straight.
Touch floor outside left foot, between feet, press once then outside right foot.
Circle bend backwards as far as possible, reverse direction after half the number of counts.
Do not strain to keep knees straight.
Keep arms above head and make full circle, bending backward past vertical each time.', 'c5_ex1.png'),
  (5, 2, 'Sit-up (Elbows to Knees)', 'Back lying, legs straight, feet together, hands clasped behind head.
Sit up and raise legs in bent position at same time twist to touch right elbow to left knee.
This completes one movement.
Alternate the direction of twist each time. Keep feet off floor when elbow touches knee.', 'c5_ex2.png'),
  (5, 3, 'Back Extension (Arms Extended Overhead)', 'Front lying, arms extended overhead.
Raise arms, head, chest and both legs as high as possible.
Keep legs and arms straight.
Chest and both thighs completely off floor.', 'c5_ex3.png'),
  (5, 4, 'Push-up with Clap', 'Front lying, hands under shoulder, palms flat on floor.
Push off floor and clap hands before returning to starting position. 
Keep body straight during the entire movement.
Hand clap must be heard.', 'c5_ex4.png'),
  (5, 5, 'Run & Semi-Spread Eagle Jumps', 'Stationary run - (count a step each time left foot touches floor. Lift feet approximately 4 inches off floor).
Every 75 steps do 10 "semi-spread eagle jumps". Repeat this sequence until required number of steps is completed.

Semi-spread eagle jumps:
Feet together, drop to a half crouch position hands on knees with arms straight.
Jump up to feet astride swing arms overhead in mid-air, return directly to starting ppositionon landing.
Raise hands above head level, spread feet at least shoulder width apart in astride position before landing with feet together.', 'c5_ex5.png'),
  (5, 6, 'Run 1 Mile (1.6km)', 'Run the specified distance on a level track or treadmill. Maintain a steady pace aimed at meeting the target time.', 'run.png'),
  (6, 1, 'Toe Touch (to Sides and Circle)', 'Feet astride, arms upward, hands reversed clasped, arms straight.
Touch floor outside left foot, between feet, press once then outside right foot.
Circle bend backwards as far as possible, reverse direction after half the number of counts.
Keeps hands reversed clasped at all times.', 'c6_ex1.png'),
  (6, 2, 'Sit-up (Touch Toes - Pike)', 'Back lying, legs straight, feet together, hands straight over the head.
Sit up and at the same time lifting both legs to touch the toes in a pike (V) position.
Keep feet together, legs and arms straight, all of the upper back and legs clear floor, fingers touch toes each time.', 'c6_ex2.png'),
  (6, 3, 'Back Extension (Arms Extended Overhead)', 'Front lying, arms extended overhead.
Raise arms, head, chest and both legs as high as possible, then press back once.
Keep legs and arms straight.
Chest and both thighs completely off floor.', 'c6_ex3.png'),
  (6, 4, 'Push-up with Chest Slap', 'Front lying, hands under shoulder, palms flat on floor.
Push off floor and slap chest before returning to starting position.
Keep body straight during the entire movement.
Chest slap must be heard.', 'c6_ex4.png'),
  (6, 5, 'Run & Jack Jumps', 'Stationary run - (count a step each time left foot touches floor. Lift feet approximately 4 inches off floor).
Every 75 steps do 10 "jack jumps". Repeat this sequence until required number of steps is completed.

Jack jumps:
Feet together, knees bent, sit on heels, finger tips touch floor. 
Jump up, raise legs waist high, keep legs straight and touch toes in midair. 
Keep legs straight, raise feet level to "standing waist height". Touch toes each time.', 'c6_ex5.png'),
  (6, 6, 'Run 1 Mile (1.6km)', 'Run the specified distance on a level track or treadmill. Maintain a steady pace aimed at meeting the target time.', 'run.png');
