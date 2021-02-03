@extends('_layouts.base')
@section('body')
<article class="serif prose">
  <header><h1 class="font-size-4.5 bold sans-serif">{{ $page->title }}</h1></header>
  @yield('content')
</article>
@endsection
