<?php
/**
 * @var Goridge\RelayInterface $relay
 */

ini_set('display_errors', 'stderr');
require __DIR__ . "/vendor/autoload.php";

use Spiral\Goridge;
use Spiral\RoadRunner;

$relay = new Goridge\StreamRelay(STDIN, STDOUT);
$rr = new RoadRunner\Worker($relay);


while ($in = $rr->waitPayload()) {
    try {
        // just echo back the request body
        $stderr = fopen( 'php://stderr', 'w' );
        fwrite($stderr, "Written through the PHP error stream" );
        fclose($stderr);
//        fwrite(STDERR, "warning: some weird php error, THIS IS PHP, I'm THE KING :) ");
//        echo "Worker received: " . (string)$in->body . "\n";
        $rr->respond(new RoadRunner\Payload((string)$in->body));
    } catch (\Throwable $e) {
        $rr->error((string)$e);
    }
}