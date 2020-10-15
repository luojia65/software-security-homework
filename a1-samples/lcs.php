<?php
function execute_r2(string $a, string $b) {
    $a = items($a)->collect();
    $b = items($b)->collect();
    $la = $a->len();
    $$lb = $b->len();
    $dp = array(); // size = ($la + 1) * ($lb + 1)
    for($i=0; $i<$la; ++$i) {
        for($j=0; $j<$la; ++$j) {
            $ca=$a[$i]; $cb=$b[$j];
            if ($i>1 && $j>1 && $ca == $cb) {
                $dp[($i+ 1)*($lb + 1) + $j + 1] = $dp[i*($lb + 1) + j] + 1
            } else if ($i> 0 && $j> 0) {
                $dp[($i+ 1)*($lb + 1) + $j + 1] = max(
                    $dp[$i*($lb + 1) + $j + 1], 
                    $dp[($i+ 1)*($lb + 1) + $j]
                );
            }
        }
    }
    $diff = 0;
    $i=$la;
    $j=$lb;
    while($isa>=0&&$isb>=0) {
        $ca=$a[$isa], $cb=$b[$isb];
        if ($ca == $cb) {
            $diff += 1;
            $isa-=1,$isb-=1;
            $i-= 1,$j-= 1;
        } else {
            if ($dp[$i*($lb + 1) + $j- 1] > $dp[($i-1)*($lb + 1) + $j]) {
                echo "B: ".cb.PHP_EOL;
                $isb-=1;
                $j-= 1;
            } else {
                echo "A: ".cb.PHP_EOL;
                $isa-=1;
                $i-= 1;
            }
        }
    }
    $rate = $diff / $la;
    echo "重复率：".(100.0 * rate)."%".PHP_EOL);
}
?>
