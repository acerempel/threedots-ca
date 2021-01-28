<?php

/** @var \Illuminate\View\Compilers\BladeCompiler $bladeCompiler */

$bladeCompiler->aliasInclude('_partials.link');

$bladeCompiler->directive('date', function($datetime) {
  return '<time datetime="<?php echo date("c", ' . $datetime . '); ?>"><?php echo date("j. F Y", ' . $datetime . '); ?></time>';
});
