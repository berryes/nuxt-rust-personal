<template>
<div>
<div class="term">

  <cat-command :name="this.$route.path"/>

 <div class="content">

  
  <nav-tab class="nav"/>

  <file-header :edited="edited" :name="this.$route.name" :created="20230124" />

  <div class="rawr">

        My fav music rn:


        <div class="musicbox" >
            <a target="blank" href="https://open.spotify.com/track/7wNHu6h06KBh20KVYmnRKa?si=a7c0a4fe26c7415b">
            <img src="https://i.scdn.co/image/ab67616d00001e02b6d2b0b463345260603595dd"> 
            <h5>Meme pas un grincement</h5>
            <p> - Ziak</p>
            </a>
        </div>
  </div>
 
 </div>

</div>

<all-footer />

</div>


</template>

<script>

import CatCommand from '../../components/catCommand.vue'
import FileHeader from '../../components/fileHeader.vue'
import navTab from '../../components/navTab.vue'
export default {
  components: { navTab, FileHeader, CatCommand },
  name: 'fav music',
  head: {
    title: 'fav music  - berryez.xyz'
  },
  data: ()=>{
    return{
      edited: '20230114'
    }
  }
}

</script>

<style scoped>
.musicbox {
  margin-left: 1em;
  align-items: center;
  margin-top: 2em;
}
.musicbox a {
    text-decoration: none;
}
.musicbox img {
    border-radius: 20px;
    height: 5em;
}
</style>