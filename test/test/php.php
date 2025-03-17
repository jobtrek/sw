<?php
function count_to(int $num) {
    /*
    Write your code here
    */
    // --sw-wipe--
    for ($i = 0; $i < $num; $i++) {
        echo $i;
    }
    // --sw-wipe--
}

class Lib {
    //Write your code here
    public static function len($content): int {
        // --sw-wipe--
        if (is_array($content)) {
            return count($content);
        }
        return strlen($content);
        // --sw-wipe--
    }
}
?>

