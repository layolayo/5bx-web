-- Clean up legacy concatenated verdicts in workout_sessions
UPDATE workout_sessions
SET 
  verdict_strength = regexp_replace(verdict_strength, $$^Strength *\((.+?)\) *(?:\||/) *Cardio *\((.*)\)$$, $$\1$$, 'i'),
  verdict_cardio = regexp_replace(verdict_cardio, $$^Strength *\((.+?)\) *(?:\||/) *Cardio *\((.*)\)$$, $$\2$$, 'i')
WHERE verdict_strength ILIKE '%Cardio%';

UPDATE workout_sessions
SET 
  verdict_strength = regexp_replace(verdict_strength, $$^MANUAL SET:\s*S\((.*?)\)\s*\|\s*C\((.*?)\)$$, $$Manual Set: \1$$, 'i'),
  verdict_cardio = regexp_replace(verdict_cardio, $$^MANUAL SET:\s*S\((.*?)\)\s*\|\s*C\((.*?)\)$$, $$Manual Set: \2$$, 'i')
WHERE verdict_strength ILIKE '%MANUAL SET%';
