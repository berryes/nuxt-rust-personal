<template>
  <div>
      <p> <!--[guest@ubuntu ~] $--></p> <div class="type" >cat /berryez.xyz{{name}}.html</div>
  </div>
</template>

<script>
export default {
  props: ['name'],
  data(){
    return {
      lengus: `cat /berryez.xyz/${this.name}.html`.length
    }
  }
}
</script>

<style>
.type {
  width: 40ch;
  animation: type 2s steps(30), blink 1s step-end infinite alternate;
  white-space: nowrap;
  overflow: hidden;
  border-right: 3px solid green;
}

@keyframes type {
  from {
    width: 0
  }
}

@keyframes content {
    from {
        opacity: 0;
    }
}

@keyframes blink {
  50% {
    border-color: transparent
  }
}
</style>