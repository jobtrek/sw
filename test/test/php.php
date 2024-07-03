<?php
function count_to(int $num) {
    /*
    Write your code here
    */
    for ($i = 0; $i < $num; $i++) {
        echo $i;
    }
}

class Lib {
    //Write your code here
    public static function len($content): int {
        if (is_array($content)) {
            return count($content);
        }
        return strlen($content);
    }
}
?>

