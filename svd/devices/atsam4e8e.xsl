<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <xsl:include href="include/common.xsl"/>
  
  
  <!-- The CHIPID:CIDR:ARCH contains duplicate. the datasheet only has 1 possible value -->
  <xsl:template match="/device/peripherals/peripheral[name='CHIPID']/registers/register[name='CIDR']/fields/field[name='ARCH']/enumeratedValues">
    <enumeratedValue>
      <name>SAM4E</name>
      <description>SAM4E series</description>
      <value>0x3C</value>
    </enumeratedValue>
  </xsl:template>
  <!-- Adds missing PID -->
  <xsl:template match="/device/peripherals/peripheral[name='PMC']/registers/register[name='PMC_PCER0']/fields">
    <xsl:copy>
      <field>
        <name>PID7</name>
        <description>Peripheral Clock 7 Enable</description>
        <bitOffset>7</bitOffset>
        <bitWidth>1</bitWidth>
        <access>write-only</access>
      </field>
      <field>
        <name>PID8</name>
        <description>Peripheral Clock 8 Enable</description>
        <bitOffset>8</bitOffset>
        <bitWidth>1</bitWidth>
        <access>write-only</access>
      </field>
      <xsl:apply-templates/>
    </xsl:copy>
  </xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='PMC']/registers/register[name='PMC_PCDR0']/fields">
    <xsl:copy>
      <field>
        <name>PID7</name>
        <description>Peripheral Clock 7 Enable</description>
        <bitOffset>7</bitOffset>
        <bitWidth>1</bitWidth>
        <access>write-only</access>
      </field>
      <field>
        <name>PID8</name>
        <description>Peripheral Clock 8 Enable</description>
        <bitOffset>8</bitOffset>
        <bitWidth>1</bitWidth>
        <access>write-only</access>
      </field>
      <xsl:apply-templates/>
    </xsl:copy>
  </xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='PMC']/registers/register[name='PMC_PCSR0']/fields">
    <xsl:copy>
      <field>
        <name>PID7</name>
        <description>Peripheral Clock 7 Enable</description>
        <bitOffset>7</bitOffset>
        <bitWidth>1</bitWidth>
        <access>write-only</access>
      </field>
      <field>
        <name>PID8</name>
        <description>Peripheral Clock 8 Enable</description>
        <bitOffset>8</bitOffset>
        <bitWidth>1</bitWidth>
        <access>write-only</access>
      </field>
      <xsl:apply-templates/>
    </xsl:copy>
  </xsl:template>
  <!-- Adds a variant to allow safe code to enter the password.  -->
  <xsl:template match="/device/peripherals/peripheral[name='PMC']/registers/register[name='CKGR_MOR']/fields/field[name='KEY']">
    <xsl:copy>
      <xsl:apply-templates/>
      <enumeratedValues>
        <enumeratedValue>
          <name>PASSWD</name>
          <description>Password</description>
          <value>0x37</value>
        </enumeratedValue>
      </enumeratedValues>
    </xsl:copy>
  </xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='SUPC']/registers/register[name='CR']/fields/field[name='KEY']">
    <xsl:copy>
      <xsl:apply-templates/>
      <enumeratedValues>
        <enumeratedValue>
          <name>PASSWD</name>
          <description>Password</description>
          <value>0xA5</value>
        </enumeratedValue>
      </enumeratedValues>
    </xsl:copy>
  </xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='RSTC']/registers/register[name='CR']/fields/field[name='KEY']">
    <xsl:copy>
      <xsl:apply-templates/>
      <enumeratedValues>
        <enumeratedValue>
          <name>PASSWD</name>
          <description>Password</description>
          <value>0xA5</value>
        </enumeratedValue>
      </enumeratedValues>
    </xsl:copy>
  </xsl:template>
  <!-- Adds a variant to allow safe code to enter programable clock prescaler.  -->
  <xsl:template match="/device/peripherals/peripheral[name='PMC']/registers/register[name='PMC_PCK%s']/fields/field[name='PRES']">
    <xsl:copy>
      <xsl:apply-templates/>
      <enumeratedValues>
        <enumeratedValue>
          <name>CLK_1</name>
          <description>Selected clock</description>
          <value>0</value>
        </enumeratedValue>
        <enumeratedValue>
          <name>CLK_2</name>
          <description>Selected clock divided by 2</description>
          <value>1</value>
        </enumeratedValue>
        <enumeratedValue>
          <name>CLK_4</name>
          <description>Selected clock divided by 4</description>
          <value>2</value>
        </enumeratedValue>
        <enumeratedValue>
          <name>CLK_8</name>
          <description>Selected clock divided by 8</description>
          <value>3</value>
        </enumeratedValue>
        <enumeratedValue>
          <name>CLK_16</name>
          <description>Selected clock divided by 16</description>
          <value>4</value>
        </enumeratedValue>
        <enumeratedValue>
          <name>CLK_32</name>
          <description>Selected clock divided by 32</description>
          <value>5</value>
        </enumeratedValue>
        <enumeratedValue>
          <name>CLK_64</name>
          <description>Selected clock divided by 64</description>
          <value>6</value>
        </enumeratedValue>
      </enumeratedValues>
    </xsl:copy>
  </xsl:template>
</xsl:stylesheet>
