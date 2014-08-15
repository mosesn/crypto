package com.mosesn.crypto

import org.apache.commons.codec.binary.{Base64, Hex}
import java.nio.ByteBuffer

object Driver {
  def main(args: Array[String]) {
    if (args.length != 1) {
      System.exit(1)
    }
    println(hexToBase64(args(0)))
  }

  def hexToBase64(arg: String): String = {
    val bytes = Hex.decodeHex(arg.toCharArray)
    Base64.encodeBase64String(bytes)
  }
}
