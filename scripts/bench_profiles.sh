#!/bin/bash

PROFILE=" release-no-lto-no-opt release-no-lto-opt-1 release-no-lto-opt-2 release-no-lto-opt-3 release-no-lto-opt-s release-no-lto-opt-z release-lto-no-opt release-lto-opt-1 release-lto-opt-2 release-lto-opt-3 release-lto-opt-s release-lto-opt-z" 

FILE=results
TEST=game_100000_secs
echo "" > $FILE

for i in $PROFILE;
do 
   echo "Profile: $i" | tee -a $FILE
   echo "Compilation" | tee -a $FILE
   rm -rf target
   { time $(cargo test $TEST --no-run --profile "$i" &> /dev/null); } &>>$FILE 
   echo ""
   echo "Performance" | tee -a $FILE
   for j in $(seq 1 3);
   do
      echo "Execution: $j" | tee -a $FILE
      { time $(cargo test $TEST --profile "$i" &> /dev/null); } &>>$FILE 
   done
   echo "" | tee -a $FILE
   echo "--------------------------------------------------------------------" | tee -a $FILE
   echo "" | tee -a $FILE
done

# cargo test game_million_secs --profile release-lto-opt-1 -- --show-output

# cargo test game_million_secs --profile release-lto-opt-2 -- --show-output

# cargo test game_million_secs --profile release-lto-opt-3 -- --show-output

# cargo test game_million_secs --profile release-lto-opt-s -- --show-output

# cargo test game_million_secs --profile release-lto-opt-z -- --show-output

# cargo test game_million_secs --profile release-no-lto-no-opt -- --show-output

# cargo test game_million_secs --profile release-no-lto-opt-1 -- --show-output

# cargo test game_million_secs --profile release-no-lto-opt-2 -- --show-output

# cargo test game_million_secs --profile release-no-lto-opt-2 -- --show-output

# cargo test game_million_secs --profile release-no-lto-opt-s -- --show-output

# cargo test game_million_secs --profile release-no-lto-opt-z -- --show-output
