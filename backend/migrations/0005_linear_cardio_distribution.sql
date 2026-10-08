-- Migration 0005: Linear Cardio Roadwork Distribution (D- to A+ Anchored Progression)
-- Updates exercise_charts ex5_run and ex5_walk targets to eliminate flat 3-rung plateaus
-- and distribute target time ceilings smoothly across all 12 rungs per chart.

-- Chart 1: Run 0.5 mi (480s down to 330s) & Walk 1.0 mi (1260s down to 1020s)
UPDATE exercise_charts SET ex5_run = 480, ex5_walk = 1260 WHERE chart = 1 AND level = 1;
UPDATE exercise_charts SET ex5_run = 466, ex5_walk = 1238 WHERE chart = 1 AND level = 2;
UPDATE exercise_charts SET ex5_run = 453, ex5_walk = 1216 WHERE chart = 1 AND level = 3;
UPDATE exercise_charts SET ex5_run = 439, ex5_walk = 1195 WHERE chart = 1 AND level = 4;
UPDATE exercise_charts SET ex5_run = 425, ex5_walk = 1173 WHERE chart = 1 AND level = 5;
UPDATE exercise_charts SET ex5_run = 412, ex5_walk = 1151 WHERE chart = 1 AND level = 6;
UPDATE exercise_charts SET ex5_run = 398, ex5_walk = 1129 WHERE chart = 1 AND level = 7;
UPDATE exercise_charts SET ex5_run = 385, ex5_walk = 1107 WHERE chart = 1 AND level = 8;
UPDATE exercise_charts SET ex5_run = 371, ex5_walk = 1085 WHERE chart = 1 AND level = 9;
UPDATE exercise_charts SET ex5_run = 357, ex5_walk = 1064 WHERE chart = 1 AND level = 10;
UPDATE exercise_charts SET ex5_run = 344, ex5_walk = 1042 WHERE chart = 1 AND level = 11;
UPDATE exercise_charts SET ex5_run = 330, ex5_walk = 1020 WHERE chart = 1 AND level = 12;

-- Chart 2: Run 1.0 mi (630s down to 540s) & Walk 2.0 mi (2100s down to 1800s)
UPDATE exercise_charts SET ex5_run = 630, ex5_walk = 2100 WHERE chart = 2 AND level = 1;
UPDATE exercise_charts SET ex5_run = 622, ex5_walk = 2073 WHERE chart = 2 AND level = 2;
UPDATE exercise_charts SET ex5_run = 614, ex5_walk = 2045 WHERE chart = 2 AND level = 3;
UPDATE exercise_charts SET ex5_run = 605, ex5_walk = 2018 WHERE chart = 2 AND level = 4;
UPDATE exercise_charts SET ex5_run = 597, ex5_walk = 1991 WHERE chart = 2 AND level = 5;
UPDATE exercise_charts SET ex5_run = 589, ex5_walk = 1964 WHERE chart = 2 AND level = 6;
UPDATE exercise_charts SET ex5_run = 581, ex5_walk = 1936 WHERE chart = 2 AND level = 7;
UPDATE exercise_charts SET ex5_run = 573, ex5_walk = 1909 WHERE chart = 2 AND level = 8;
UPDATE exercise_charts SET ex5_run = 565, ex5_walk = 1882 WHERE chart = 2 AND level = 9;
UPDATE exercise_charts SET ex5_run = 556, ex5_walk = 1855 WHERE chart = 2 AND level = 10;
UPDATE exercise_charts SET ex5_run = 548, ex5_walk = 1827 WHERE chart = 2 AND level = 11;
UPDATE exercise_charts SET ex5_run = 540, ex5_walk = 1800 WHERE chart = 2 AND level = 12;

-- Chart 3: Run 1.0 mi (525s down to 480s) & Jog 2.0 mi (1740s down to 1500s)
UPDATE exercise_charts SET ex5_run = 525, ex5_walk = 1740 WHERE chart = 3 AND level = 1;
UPDATE exercise_charts SET ex5_run = 521, ex5_walk = 1718 WHERE chart = 3 AND level = 2;
UPDATE exercise_charts SET ex5_run = 517, ex5_walk = 1696 WHERE chart = 3 AND level = 3;
UPDATE exercise_charts SET ex5_run = 513, ex5_walk = 1675 WHERE chart = 3 AND level = 4;
UPDATE exercise_charts SET ex5_run = 509, ex5_walk = 1653 WHERE chart = 3 AND level = 5;
UPDATE exercise_charts SET ex5_run = 505, ex5_walk = 1631 WHERE chart = 3 AND level = 6;
UPDATE exercise_charts SET ex5_run = 500, ex5_walk = 1609 WHERE chart = 3 AND level = 7;
UPDATE exercise_charts SET ex5_run = 496, ex5_walk = 1587 WHERE chart = 3 AND level = 8;
UPDATE exercise_charts SET ex5_run = 492, ex5_walk = 1565 WHERE chart = 3 AND level = 9;
UPDATE exercise_charts SET ex5_run = 488, ex5_walk = 1544 WHERE chart = 3 AND level = 10;
UPDATE exercise_charts SET ex5_run = 484, ex5_walk = 1522 WHERE chart = 3 AND level = 11;
UPDATE exercise_charts SET ex5_run = 480, ex5_walk = 1500 WHERE chart = 3 AND level = 12;

-- Chart 4: Run 1.0 mi (465s down to 420s) & Jog 2.0 mi (1380s down to 1140s)
UPDATE exercise_charts SET ex5_run = 465, ex5_walk = 1380 WHERE chart = 4 AND level = 1;
UPDATE exercise_charts SET ex5_run = 461, ex5_walk = 1358 WHERE chart = 4 AND level = 2;
UPDATE exercise_charts SET ex5_run = 457, ex5_walk = 1336 WHERE chart = 4 AND level = 3;
UPDATE exercise_charts SET ex5_run = 453, ex5_walk = 1315 WHERE chart = 4 AND level = 4;
UPDATE exercise_charts SET ex5_run = 449, ex5_walk = 1293 WHERE chart = 4 AND level = 5;
UPDATE exercise_charts SET ex5_run = 445, ex5_walk = 1271 WHERE chart = 4 AND level = 6;
UPDATE exercise_charts SET ex5_run = 440, ex5_walk = 1249 WHERE chart = 4 AND level = 7;
UPDATE exercise_charts SET ex5_run = 436, ex5_walk = 1227 WHERE chart = 4 AND level = 8;
UPDATE exercise_charts SET ex5_run = 432, ex5_walk = 1205 WHERE chart = 4 AND level = 9;
UPDATE exercise_charts SET ex5_run = 428, ex5_walk = 1184 WHERE chart = 4 AND level = 10;
UPDATE exercise_charts SET ex5_run = 424, ex5_walk = 1162 WHERE chart = 4 AND level = 11;
UPDATE exercise_charts SET ex5_run = 420, ex5_walk = 1140 WHERE chart = 4 AND level = 12;

-- Chart 5: Run 1.0 mi (420s down to 360s)
UPDATE exercise_charts SET ex5_run = 420, ex5_walk = 0 WHERE chart = 5 AND level = 1;
UPDATE exercise_charts SET ex5_run = 415, ex5_walk = 0 WHERE chart = 5 AND level = 2;
UPDATE exercise_charts SET ex5_run = 409, ex5_walk = 0 WHERE chart = 5 AND level = 3;
UPDATE exercise_charts SET ex5_run = 404, ex5_walk = 0 WHERE chart = 5 AND level = 4;
UPDATE exercise_charts SET ex5_run = 398, ex5_walk = 0 WHERE chart = 5 AND level = 5;
UPDATE exercise_charts SET ex5_run = 393, ex5_walk = 0 WHERE chart = 5 AND level = 6;
UPDATE exercise_charts SET ex5_run = 387, ex5_walk = 0 WHERE chart = 5 AND level = 7;
UPDATE exercise_charts SET ex5_run = 382, ex5_walk = 0 WHERE chart = 5 AND level = 8;
UPDATE exercise_charts SET ex5_run = 376, ex5_walk = 0 WHERE chart = 5 AND level = 9;
UPDATE exercise_charts SET ex5_run = 371, ex5_walk = 0 WHERE chart = 5 AND level = 10;
UPDATE exercise_charts SET ex5_run = 365, ex5_walk = 0 WHERE chart = 5 AND level = 11;
UPDATE exercise_charts SET ex5_run = 360, ex5_walk = 0 WHERE chart = 5 AND level = 12;

-- Chart 6: Run 1.0 mi (360s down to 300s)
UPDATE exercise_charts SET ex5_run = 360, ex5_walk = 0 WHERE chart = 6 AND level = 1;
UPDATE exercise_charts SET ex5_run = 355, ex5_walk = 0 WHERE chart = 6 AND level = 2;
UPDATE exercise_charts SET ex5_run = 349, ex5_walk = 0 WHERE chart = 6 AND level = 3;
UPDATE exercise_charts SET ex5_run = 344, ex5_walk = 0 WHERE chart = 6 AND level = 4;
UPDATE exercise_charts SET ex5_run = 338, ex5_walk = 0 WHERE chart = 6 AND level = 5;
UPDATE exercise_charts SET ex5_run = 333, ex5_walk = 0 WHERE chart = 6 AND level = 6;
UPDATE exercise_charts SET ex5_run = 327, ex5_walk = 0 WHERE chart = 6 AND level = 7;
UPDATE exercise_charts SET ex5_run = 322, ex5_walk = 0 WHERE chart = 6 AND level = 8;
UPDATE exercise_charts SET ex5_run = 316, ex5_walk = 0 WHERE chart = 6 AND level = 9;
UPDATE exercise_charts SET ex5_run = 311, ex5_walk = 0 WHERE chart = 6 AND level = 10;
UPDATE exercise_charts SET ex5_run = 305, ex5_walk = 0 WHERE chart = 6 AND level = 11;
UPDATE exercise_charts SET ex5_run = 300, ex5_walk = 0 WHERE chart = 6 AND level = 12;
